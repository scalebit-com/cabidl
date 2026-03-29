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
/// (e.g. Graphviz DOT, Mermaid). Takes a parsed System model and returns the
/// diagram content as a string.
pub trait DiagramProvider {
    /// Returns the diagram type identifier (e.g. "graphviz", "mermaid").
    /// Used to match against the `-f/--format` CLI argument.
    fn diagram_type(&self) -> &str;

    /// Generate diagram content from a parsed System model.
    ///
    /// - `system`: The parsed CABIDL system model.
    /// - `diagram_sub_type`: Optional sub-type qualifier within the format
    ///   (e.g. "dark"/"light" for Graphviz, "c4"/"class" for Mermaid).
    ///   Each provider defines its own valid sub-types and default.
    fn generate(&self, system: &System, diagram_sub_type: Option<&str>) -> Result<String, DiagramError>;
}
