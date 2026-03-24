use std::collections::HashSet;

use crate::types::{CabidlDocument, ValidationError};

pub fn validate(doc: &CabidlDocument, file: &str) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Validate boundary exposure values
    for boundary in &doc.boundaries {
        if let Some(ref exposure) = boundary.exposure {
            if exposure != "external" && exposure != "internal" {
                errors.push(ValidationError {
                    message: format!(
                        "Invalid exposure value '{}' in boundary '{}' (must be 'external' or 'internal')",
                        exposure, boundary.name
                    ),
                    file: file.to_string(),
                    line: boundary.line,
                });
            }
        }
    }

    // Validate boundary reference integrity
    let defined_boundaries: HashSet<&str> = doc.boundaries.iter().map(|b| b.name.as_str()).collect();

    for component in &doc.components {
        for boundary_name in &component.provides {
            if !defined_boundaries.contains(boundary_name.as_str()) {
                errors.push(ValidationError {
                    message: format!(
                        "Component '{}' provides undefined boundary '{}'",
                        component.name, boundary_name
                    ),
                    file: file.to_string(),
                    line: component.line,
                });
            }
        }
        for boundary_name in &component.requires {
            if !defined_boundaries.contains(boundary_name.as_str()) {
                errors.push(ValidationError {
                    message: format!(
                        "Component '{}' requires undefined boundary '{}'",
                        component.name, boundary_name
                    ),
                    file: file.to_string(),
                    line: component.line,
                });
            }
        }
    }

    errors
}
