use std::path::Path;

/// A parsed CABIDL document containing all sections.
pub struct CabidlDocument {
    /// The system definition (exactly one per document).
    pub system: SystemBlock,
    /// All boundary definitions.
    pub boundaries: Vec<BoundaryBlock>,
    /// All component definitions.
    pub components: Vec<ComponentBlock>,
}

pub struct SystemBlock {
    pub name: String,
}

pub struct BoundaryBlock {
    pub name: String,
    pub exposure: Option<String>,
    pub specification_path: Option<String>,
    pub specification_type: Option<String>,
}

pub struct ComponentBlock {
    pub name: String,
    pub technology: Option<String>,
    pub provides: Vec<String>,
    pub requires: Vec<String>,
}

/// A validation error with location context.
pub struct ValidationError {
    pub message: String,
    pub file: String,
    pub line: Option<usize>,
}

/// The CabidlParser boundary trait.
///
/// Provides parsing, include resolution, and validation of CABIDL documents.
pub trait CabidlParser {
    /// Parse a CABIDL markdown file into a structured document.
    /// Resolves `<!-- @include -->` directives recursively.
    fn parse(&self, path: &Path) -> Result<CabidlDocument, Vec<ValidationError>>;

    /// Validate a CABIDL document.
    /// Checks YAML block structure, schema conformance, and boundary reference integrity.
    /// Returns an empty Vec on success.
    fn validate(&self, path: &Path) -> Vec<ValidationError>;
}
