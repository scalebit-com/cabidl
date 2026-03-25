use std::path::Path;

use cabidl_parser::System;
use cabidl_diagram_provider::DiagramError;

/// The Diagram boundary trait.
///
/// Orchestrates diagram generation: selects a DiagramProvider based on
/// the requested type, generates the diagram content, and writes the
/// result to the output file.
pub trait Diagram {
    /// Generate a diagram from a parsed System model and write it to a file.
    ///
    /// - `system`: The parsed CABIDL system model.
    /// - `diagram_type`: The type of diagram to generate (e.g. "graphviz").
    /// - `output_file`: Path where the diagram content will be written.
    fn generate(
        &self,
        system: &System,
        diagram_type: &str,
        output_file: &Path,
    ) -> Result<(), DiagramError>;
}
