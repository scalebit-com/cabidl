use std::path::Path;

use cabidl_diagram::Diagram;
use cabidl_diagram_provider::{DiagramError, DiagramProvider};
use cabidl_filesystem::Filesystem;
use cabidl_parser::System;

pub struct DiagramImpl {
    providers: Vec<Box<dyn DiagramProvider>>,
    fs: Box<dyn Filesystem>,
}

impl DiagramImpl {
    pub fn new(providers: Vec<Box<dyn DiagramProvider>>, fs: Box<dyn Filesystem>) -> Self {
        Self { providers, fs }
    }
}

impl Diagram for DiagramImpl {
    fn generate(
        &self,
        system: &System,
        diagram_type: &str,
        output_file: &Path,
    ) -> Result<(), DiagramError> {
        let provider = self
            .providers
            .iter()
            .find(|p| p.diagram_type() == diagram_type)
            .ok_or_else(|| DiagramError {
                message: format!("Unknown diagram type: '{}'", diagram_type),
            })?;

        let content = provider.generate(system)?;

        self.fs
            .write_string(output_file, &content)
            .map_err(|e| DiagramError {
                message: format!(
                    "Failed to write output file '{}': {}",
                    output_file.display(),
                    e
                ),
            })?;

        Ok(())
    }
}
