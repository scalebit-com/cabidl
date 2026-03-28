use std::sync::Arc;

use crate::Boundary;

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
