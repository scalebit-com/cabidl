use std::path::Path;

use crate::{System, ValidationError};

/// The CabidlParser boundary trait.
///
/// Parses CABIDL markdown into a System model with Arc-linked boundaries
/// and components. The resulting graph is the internal representation of
/// the specification and can be used for validation, display, or generation.
pub trait CabidlParser {
    /// Parse a CABIDL markdown file into a System model.
    /// Resolves `<!-- @include -->` directives recursively.
    fn parse(&self, path: &Path) -> Result<System, Vec<ValidationError>>;

    /// Parse a CABIDL document from an already-resolved content string.
    /// Pure function: string in, System model out.
    /// Primary entry point for testing without filesystem access.
    fn parse_content(&self, content: &str, file: &str) -> Result<System, Vec<ValidationError>>;

    /// Validate a parsed System model.
    /// Checks boundary exposure values, name uniqueness, and referential integrity.
    /// Returns an empty Vec on success.
    fn validate(&self, system: &System, file: &str) -> Vec<ValidationError>;
}
