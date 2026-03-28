use cabidl_ai_provider::AiProvider;
use cabidl_diagram::Diagram;
use cabidl_init::Init;
use cabidl_parser::CabidlParser;

/// The Wiring boundary trait.
///
/// Composition root that provides access to all domain boundaries.
/// Isolates the Cli from concrete implementation types so that tests
/// can inject in-memory implementations.
pub trait Wiring {
    /// Returns the parser for reading and validating CABIDL documents.
    fn parser(&self) -> &dyn CabidlParser;

    /// Returns the diagram orchestrator for generating diagrams.
    fn diagram(&self) -> &dyn Diagram;

    /// Returns the AI tool provider for skill installation and project init.
    fn ai_provider(&self) -> &dyn AiProvider;

    /// Returns the project initializer for template listing and scaffolding.
    fn init(&self) -> &dyn Init;
}
