use std::collections::HashSet;
use std::path::{Path, PathBuf};

use regex::Regex;

use crate::filesystem::Filesystem;
use crate::types::*;

/// Resolve all `<!-- @include -->` directives and return the unified markdown string.
pub fn resolve(
    fs: &dyn Filesystem,
    path: &Path,
) -> Result<String, Vec<ValidationError>> {
    let canonical = fs.canonicalize(path).map_err(|e| {
        vec![ValidationError {
            message: format!("Failed to resolve path '{}': {}", path.display(), e),
            file: path.display().to_string(),
            line: None,
        }]
    })?;

    let content = fs.read_to_string(&canonical).map_err(|e| {
        vec![ValidationError {
            message: format!("Failed to read file '{}': {}", path.display(), e),
            file: path.display().to_string(),
            line: None,
        }]
    })?;

    let mut visited = HashSet::new();
    visited.insert(canonical.clone());
    resolve_includes(fs, &content, &canonical, &mut visited)
}

fn resolve_includes(
    fs: &dyn Filesystem,
    content: &str,
    current_file: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<String, Vec<ValidationError>> {
    let include_re = Regex::new(r"^<!--\s*@include\s+(.*?)\s*-->$").unwrap();
    let mut errors = Vec::new();
    let mut result_lines = Vec::new();
    let mut in_fence = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Track fenced code blocks to avoid matching inside them
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            result_lines.push(line.to_string());
            continue;
        }

        if in_fence {
            result_lines.push(line.to_string());
            continue;
        }

        // Only match include directives on standalone lines
        if let Some(caps) = include_re.captures(trimmed) {
            let include_path_str = caps.get(1).unwrap().as_str().trim();
            let include_path = Path::new(include_path_str);

            match fs.resolve_path(current_file, include_path) {
                Ok(resolved) => {
                    if visited.contains(&resolved) {
                        errors.push(ValidationError {
                            message: format!(
                                "Circular include detected: {}",
                                resolved.display()
                            ),
                            file: current_file.display().to_string(),
                            line: None,
                        });
                        continue;
                    }
                    visited.insert(resolved.clone());
                    match fs.read_to_string(&resolved) {
                        Ok(included_content) => {
                            match resolve_includes(
                                fs,
                                &included_content,
                                &resolved,
                                visited,
                            ) {
                                Ok(resolved_content) => {
                                    result_lines.push(resolved_content);
                                }
                                Err(mut errs) => errors.append(&mut errs),
                            }
                        }
                        Err(e) => {
                            errors.push(ValidationError {
                                message: format!(
                                    "Failed to read included file '{}': {}",
                                    include_path_str, e
                                ),
                                file: current_file.display().to_string(),
                                line: None,
                            });
                        }
                    }
                }
                Err(e) => {
                    errors.push(ValidationError {
                        message: format!(
                            "Failed to resolve include path '{}': {}",
                            include_path_str, e
                        ),
                        file: current_file.display().to_string(),
                        line: None,
                    });
                }
            }
        } else {
            result_lines.push(line.to_string());
        }
    }

    if errors.is_empty() {
        Ok(result_lines.join("\n"))
    } else {
        Err(errors)
    }
}

/// Parse a CABIDL markdown file into a structured document.
pub fn parse(
    fs: &dyn Filesystem,
    path: &Path,
) -> Result<CabidlDocument, Vec<ValidationError>> {
    let file_str = path.display().to_string();
    let content = resolve(fs, path)?;
    parse_content(&content, &file_str)
}

/// Parse CABIDL document from an already-resolved content string.
/// This is a pure function: string in, structured result out.
pub fn parse_content(
    content: &str,
    file: &str,
) -> Result<CabidlDocument, Vec<ValidationError>> {
    let yaml_blocks = extract_yaml_blocks(content);

    let mut system: Option<SystemBlock> = None;
    let mut boundaries = Vec::new();
    let mut components = Vec::new();
    let mut errors = Vec::new();

    for (yaml_str, line_num) in &yaml_blocks {
        // First pass: determine the kind
        let value: serde_yaml::Value = match serde_yaml::from_str(yaml_str) {
            Ok(v) => v,
            Err(e) => {
                errors.push(ValidationError {
                    message: format!("Invalid YAML: {}", e),
                    file: file.to_string(),
                    line: Some(*line_num),
                });
                continue;
            }
        };

        let kind = match value.get("kind").and_then(|k| k.as_str()) {
            Some(k) => k.to_string(),
            None => {
                errors.push(ValidationError {
                    message: "YAML block missing required 'kind' field".to_string(),
                    file: file.to_string(),
                    line: Some(*line_num),
                });
                continue;
            }
        };

        match kind.as_str() {
            "system" => {
                match serde_yaml::from_str::<SystemYaml>(yaml_str) {
                    Ok(s) => {
                        if system.is_some() {
                            errors.push(ValidationError {
                                message: "Multiple system blocks found (exactly one required)"
                                    .to_string(),
                                file: file.to_string(),
                                line: Some(*line_num),
                            });
                        } else {
                            system = Some(SystemBlock {
                                name: s.name,
                                line: Some(*line_num),
                            });
                        }
                    }
                    Err(e) => {
                        errors.push(ValidationError {
                            message: format!("Invalid system block: {}", e),
                            file: file.to_string(),
                            line: Some(*line_num),
                        });
                    }
                }
            }
            "boundary" => {
                match serde_yaml::from_str::<BoundaryYaml>(yaml_str) {
                    Ok(b) => {
                        boundaries.push(BoundaryBlock {
                            name: b.name,
                            exposure: b.exposure,
                            specification_path: b
                                .specification
                                .as_ref()
                                .and_then(|s| s.path.clone()),
                            specification_type: b
                                .specification
                                .as_ref()
                                .and_then(|s| s.type_description.clone()),
                            line: Some(*line_num),
                        });
                    }
                    Err(e) => {
                        errors.push(ValidationError {
                            message: format!("Invalid boundary block: {}", e),
                            file: file.to_string(),
                            line: Some(*line_num),
                        });
                    }
                }
            }
            "component" => {
                match serde_yaml::from_str::<ComponentYaml>(yaml_str) {
                    Ok(c) => {
                        components.push(ComponentBlock {
                            name: c.name,
                            technology: c.technology,
                            provides: c
                                .boundaries
                                .as_ref()
                                .and_then(|b| b.provides.clone())
                                .unwrap_or_default(),
                            requires: c
                                .boundaries
                                .as_ref()
                                .and_then(|b| b.requires.clone())
                                .unwrap_or_default(),
                            line: Some(*line_num),
                        });
                    }
                    Err(e) => {
                        errors.push(ValidationError {
                            message: format!("Invalid component block: {}", e),
                            file: file.to_string(),
                            line: Some(*line_num),
                        });
                    }
                }
            }
            other => {
                errors.push(ValidationError {
                    message: format!("Unknown block kind: '{}'", other),
                    file: file.to_string(),
                    line: Some(*line_num),
                });
            }
        }
    }

    if system.is_none() && errors.is_empty() {
        errors.push(ValidationError {
            message: "No system block found in document (exactly one required)".to_string(),
            file: file.to_string(),
            line: None,
        });
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(CabidlDocument {
        system: system.unwrap(),
        boundaries,
        components,
    })
}

/// Extract YAML code blocks from markdown content.
/// Returns Vec of (yaml_content, line_number).
fn extract_yaml_blocks(content: &str) -> Vec<(String, usize)> {
    let mut blocks = Vec::new();
    let mut in_yaml = false;
    let mut in_other_fence = false;
    let mut yaml_lines = Vec::new();
    let mut yaml_start_line = 0;

    for (i, line) in content.lines().enumerate() {
        let line_num = i + 1; // 1-based
        let trimmed = line.trim();

        if in_yaml {
            if trimmed.starts_with("```") {
                // End of YAML block
                blocks.push((yaml_lines.join("\n"), yaml_start_line));
                yaml_lines.clear();
                in_yaml = false;
            } else {
                yaml_lines.push(line.to_string());
            }
        } else if in_other_fence {
            if trimmed.starts_with("```") {
                in_other_fence = false;
            }
        } else if trimmed.starts_with("```yaml") {
            in_yaml = true;
            yaml_start_line = line_num;
        } else if trimmed.starts_with("```") {
            in_other_fence = true;
        }
    }

    blocks
}
