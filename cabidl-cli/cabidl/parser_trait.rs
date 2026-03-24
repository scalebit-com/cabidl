use std::fmt;
use std::path::Path;

// ---------------------------------------------------------------------------
// Domain types — each lives in its own file in the parser/ boundary crate
// ---------------------------------------------------------------------------

// parser/src/system.rs
/// A parsed system block.
#[derive(Debug)]
pub struct SystemBlock {
    pub name: String,
    /// Line number of the ```yaml fence that opened this block (1-based).
    pub line: Option<usize>,
}

// parser/src/boundary.rs
/// A parsed boundary block.
#[derive(Debug)]
pub struct BoundaryBlock {
    pub name: String,
    pub exposure: Option<String>,
    pub specification_path: Option<String>,
    pub specification_type: Option<String>,
    /// Line number of the ```yaml fence that opened this block (1-based).
    pub line: Option<usize>,
}

// parser/src/component.rs
/// A parsed component block.
#[derive(Debug)]
pub struct ComponentBlock {
    pub name: String,
    pub technology: Option<String>,
    pub provides: Vec<String>,
    pub requires: Vec<String>,
    /// Line number of the ```yaml fence that opened this block (1-based).
    pub line: Option<usize>,
}

// parser/src/document.rs
/// A parsed CABIDL document containing all sections.
#[derive(Debug)]
pub struct CabidlDocument {
    /// The system definition (exactly one per document).
    pub system: SystemBlock,
    /// All boundary definitions.
    pub boundaries: Vec<BoundaryBlock>,
    /// All component definitions.
    pub components: Vec<ComponentBlock>,
}

// parser/src/error.rs
/// A validation error with location context.
///
/// Formats as `file:line: message` when a line number is present,
/// or `file: message` when it is not — following compiler diagnostic conventions.
#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
    pub file: String,
    pub line: Option<usize>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(f, "{}:{}: {}", self.file, line, self.message),
            None => write!(f, "{}: {}", self.file, self.message),
        }
    }
}

// parser/src/lib.rs
/// The CabidlParser boundary trait.
///
/// Provides parsing, include resolution, and validation of CABIDL documents.
pub trait CabidlParser {
    /// Parse a CABIDL markdown file into a structured document.
    /// Resolves `<!-- @include -->` directives recursively.
    fn parse(&self, path: &Path) -> Result<CabidlDocument, Vec<ValidationError>>;

    /// Parse a CABIDL document from an already-resolved content string.
    /// This is a pure function: string in, structured result out.
    /// It is the primary entry point for testing without filesystem access.
    fn parse_content(&self, content: &str, file: &str) -> Result<CabidlDocument, Vec<ValidationError>>;

    /// Validate a parsed CABIDL document.
    /// Checks boundary exposure values, name uniqueness, and referential integrity.
    /// Returns an empty Vec on success.
    fn validate(&self, doc: &CabidlDocument, file: &str) -> Vec<ValidationError>;
}
