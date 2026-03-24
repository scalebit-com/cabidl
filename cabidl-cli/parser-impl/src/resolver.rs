use std::collections::HashSet;
use std::path::{Path, PathBuf};

use regex::Regex;

use cabidl_filesystem::Filesystem;
use cabidl_parser::{System, ValidationError};

use crate::parser::parse_content;

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

/// Parse a CABIDL markdown file into a System model.
/// Resolves `<!-- @include -->` directives, then parses the unified content.
pub fn parse(
    fs: &dyn Filesystem,
    path: &Path,
) -> Result<System, Vec<ValidationError>> {
    let file_str = path.display().to_string();
    let content = resolve(fs, path)?;
    parse_content(&content, &file_str)
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
