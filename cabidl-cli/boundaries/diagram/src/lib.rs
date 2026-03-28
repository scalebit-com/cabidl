use cabidl_parser::System;
use cabidl_diagram_provider::DiagramError;

/// The Diagram boundary trait.
///
/// Orchestrates diagram generation: selects a DiagramProvider based on
/// the requested type and generates the diagram content as a string.
pub trait Diagram {
    /// Generate a diagram from a parsed System model.
    ///
    /// - `system`: The parsed CABIDL system model.
    /// - `diagram_type`: The type of diagram to generate (e.g. "graphviz").
    ///
    /// Returns the diagram content as a string. The caller is responsible
    /// for writing the content to a file via the Filesystem boundary.
    fn generate(
        &self,
        system: &System,
        diagram_type: &str,
    ) -> Result<String, DiagramError>;
}
