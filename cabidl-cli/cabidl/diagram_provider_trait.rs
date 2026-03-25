use std::fmt;

use cabidl_parser::System;

/// An error that occurs during diagram generation.
#[derive(Debug)]
pub struct DiagramError {
    pub message: String,
}

impl fmt::Display for DiagramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// The DiagramProvider boundary trait.
///
/// Each implementer generates diagram content in a specific format
/// (e.g. Graphviz DOT). Takes a parsed System model and returns the
/// diagram content as a string.
pub trait DiagramProvider {
    /// Returns the diagram type identifier (e.g. "graphviz").
    /// Used to match against the `-t/--type` CLI argument.
    fn diagram_type(&self) -> &str;

    /// Generate diagram content from a parsed System model.
    fn generate(&self, system: &System) -> Result<String, DiagramError>;
}
