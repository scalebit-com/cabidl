use std::sync::Arc;

use crate::{Boundary, Component};

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
