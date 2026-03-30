use std::path::Path;

use cabidl_filesystem::Filesystem;
use cabidl_parser::{CabidlParser, System, ValidationError};

/// Implementation of the CabidlParser boundary.
///
/// Thin wrapper that delegates to pure functions, holding only a
/// reference to a Filesystem implementation for include resolution.
pub struct CabidlParserImpl {
    fs: Box<dyn Filesystem>,
}

impl CabidlParserImpl {
    pub fn new(fs: Box<dyn Filesystem>) -> Self {
        Self { fs }
    }
}

impl CabidlParser for CabidlParserImpl {
    fn parse(&self, path: &Path) -> Result<System, Vec<ValidationError>> {
        crate::resolver::parse(&*self.fs, path)
    }

    fn parse_content(&self, content: &str, file: &str) -> Result<System, Vec<ValidationError>> {
        crate::parser::parse_content(content, file)
    }

    fn validate(&self, system: &System, file: &str) -> Vec<ValidationError> {
        let mut errors = crate::validator::validate(system, file);

        let base = std::path::Path::new(file);
        for boundary in &system.boundaries {
            if let Some(ref spec_path) = boundary.specification_path {
                match self.fs.resolve_path(base, std::path::Path::new(spec_path)) {
                    Ok(path) => {
                        if !self.fs.exists(&path) {
                            errors.push(ValidationError {
                                message: format!(
                                    "Specification path '{}' in boundary '{}' does not exist",
                                    spec_path, boundary.name
                                ),
                                file: file.to_string(),
                                line: boundary.line,
                            });
                        }
                    }
                    Err(e) => {
                        errors.push(ValidationError {
                            message: format!(
                                "Cannot resolve specification path '{}' in boundary '{}': {}",
                                spec_path, boundary.name, e
                            ),
                            file: file.to_string(),
                            line: boundary.line,
                        });
                    }
                }
            }
        }

        errors
    }
}
