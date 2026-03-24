use std::fmt;
use std::path::Path;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Domain model — a graph of the parsed CABIDL specification
//
// The parser returns a System that owns all Boundary and Component instances.
// Components hold Arc references to the boundaries they provide and require,
// making the architecture graph directly navigable without string lookups.
// ---------------------------------------------------------------------------

/// A parsed CABIDL system — the root of the architecture model.
///
/// Contains all boundaries and components. Components reference boundaries
/// via Arc, so the relationship graph can be traversed directly.
#[derive(Debug)]
pub struct System {
    pub name: String,
    pub boundaries: Vec<Arc<Boundary>>,
    pub components: Vec<Arc<Component>>,
    /// Line number of the system block (1-based).
    pub line: Option<usize>,
}

/// An architectural boundary — an interface or contract between components.
#[derive(Debug)]
pub struct Boundary {
    pub name: String,
    pub exposure: Option<String>,
    pub specification_path: Option<String>,
    pub specification_type: Option<String>,
    /// Line number of the boundary block (1-based).
    pub line: Option<usize>,
}

/// A component — a building block of the system.
///
/// Provides and requires boundaries via Arc references, mirroring the
/// `boundaries.provides` and `boundaries.requires` fields from the spec.
#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub technology: Option<String>,
    /// Boundaries this component exposes.
    pub provides: Vec<Arc<Boundary>>,
    /// Boundaries this component depends on.
    pub requires: Vec<Arc<Boundary>>,
    /// Line number of the component block (1-based).
    pub line: Option<usize>,
}

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
