use cabidl_diagram::Diagram;
use cabidl_diagram_provider::{DiagramError, DiagramProvider};
use cabidl_parser::System;

pub struct DiagramImpl {
    providers: Vec<Box<dyn DiagramProvider>>,
}

impl DiagramImpl {
    pub fn new(providers: Vec<Box<dyn DiagramProvider>>) -> Self {
        Self { providers }
    }
}

impl Diagram for DiagramImpl {
    fn generate(
        &self,
        system: &System,
        diagram_type: &str,
    ) -> Result<String, DiagramError> {
        let provider = self
            .providers
            .iter()
            .find(|p| p.diagram_type() == diagram_type)
            .ok_or_else(|| DiagramError {
                message: format!("Unknown diagram type: '{}'", diagram_type),
            })?;

        provider.generate(system)
    }
}
