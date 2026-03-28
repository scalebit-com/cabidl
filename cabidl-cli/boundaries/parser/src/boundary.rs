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
