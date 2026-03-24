use std::collections::HashSet;

use cabidl_parser::{System, ValidationError};

/// Validate a parsed System model.
/// Pure function: checks exposure values and component name uniqueness.
///
/// Boundary name uniqueness and boundary reference integrity are enforced
/// at parse time during Arc resolution, so they are not checked here.
pub fn validate(system: &System, file: &str) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Validate boundary exposure values
    for boundary in &system.boundaries {
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

    // Validate component name uniqueness
    let mut seen_components: HashSet<&str> = HashSet::new();
    for component in &system.components {
        if !seen_components.insert(&component.name) {
            errors.push(ValidationError {
                message: format!("Duplicate component name '{}'", component.name),
                file: file.to_string(),
                line: component.line,
            });
        }
    }

    errors
}
