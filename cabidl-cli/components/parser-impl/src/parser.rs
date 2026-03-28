use std::collections::HashMap;
use std::sync::Arc;

use cabidl_parser::{Boundary, Component, System, ValidationError};

use crate::yaml_types::*;

/// Parse CABIDL document from an already-resolved content string.
/// Pure function: string in, System model out.
///
/// Builds an Arc-linked graph where components reference boundaries directly.
/// Boundary name uniqueness and reference integrity are enforced at parse time.
pub fn parse_content(
    content: &str,
    file: &str,
) -> Result<System, Vec<ValidationError>> {
    let yaml_blocks = extract_yaml_blocks(content);

    let mut system_name: Option<String> = None;
    let mut system_line: Option<usize> = None;
    let mut boundaries: Vec<Arc<Boundary>> = Vec::new();
    let mut boundary_map: HashMap<String, Arc<Boundary>> = HashMap::new();
    let mut raw_components: Vec<RawComponent> = Vec::new();
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
                        if system_name.is_some() {
                            errors.push(ValidationError {
                                message: "Multiple system blocks found (exactly one required)"
                                    .to_string(),
                                file: file.to_string(),
                                line: Some(*line_num),
                            });
                        } else {
                            system_name = Some(s.name);
                            system_line = Some(*line_num);
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
                        if boundary_map.contains_key(&b.name) {
                            errors.push(ValidationError {
                                message: format!("Duplicate boundary name '{}'", b.name),
                                file: file.to_string(),
                                line: Some(*line_num),
                            });
                        } else {
                            let boundary = Arc::new(Boundary {
                                name: b.name.clone(),
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
                            boundary_map.insert(b.name, Arc::clone(&boundary));
                            boundaries.push(boundary);
                        }
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
                        raw_components.push(RawComponent {
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

    if system_name.is_none() && errors.is_empty() {
        errors.push(ValidationError {
            message: "No system block found in document (exactly one required)".to_string(),
            file: file.to_string(),
            line: None,
        });
    }

    // Resolve component boundary references into Arc links
    let mut components: Vec<Arc<Component>> = Vec::new();
    for raw in &raw_components {
        let mut provides = Vec::new();
        for name in &raw.provides {
            match boundary_map.get(name) {
                Some(boundary) => provides.push(Arc::clone(boundary)),
                None => {
                    errors.push(ValidationError {
                        message: format!(
                            "Component '{}' provides undefined boundary '{}'",
                            raw.name, name
                        ),
                        file: file.to_string(),
                        line: raw.line,
                    });
                }
            }
        }

        let mut requires = Vec::new();
        for name in &raw.requires {
            match boundary_map.get(name) {
                Some(boundary) => requires.push(Arc::clone(boundary)),
                None => {
                    errors.push(ValidationError {
                        message: format!(
                            "Component '{}' requires undefined boundary '{}'",
                            raw.name, name
                        ),
                        file: file.to_string(),
                        line: raw.line,
                    });
                }
            }
        }

        components.push(Arc::new(Component {
            name: raw.name.clone(),
            technology: raw.technology.clone(),
            provides,
            requires,
            line: raw.line,
        }));
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(System {
        name: system_name.unwrap(),
        boundaries,
        components,
        line: system_line,
    })
}

/// Temporary struct to hold component data before boundary references are resolved.
struct RawComponent {
    name: String,
    technology: Option<String>,
    provides: Vec<String>,
    requires: Vec<String>,
    line: Option<usize>,
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
