use serde::Deserialize;
use std::fmt;

// --- YAML deserialization structs ---

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemYaml {
    pub kind: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryYaml {
    pub kind: String,
    pub name: String,
    pub exposure: Option<String>,
    pub specification: Option<SpecificationYaml>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SpecificationYaml {
    pub path: Option<String>,
    pub type_description: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentYaml {
    pub kind: String,
    pub name: String,
    pub technology: Option<String>,
    pub boundaries: Option<BoundariesYaml>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundariesYaml {
    pub provides: Option<Vec<String>>,
    pub requires: Option<Vec<String>>,
}

// --- Domain types ---

#[derive(Debug)]
pub struct CabidlDocument {
    pub system: SystemBlock,
    pub boundaries: Vec<BoundaryBlock>,
    pub components: Vec<ComponentBlock>,
}

#[derive(Debug)]
pub struct SystemBlock {
    pub name: String,
    pub line: Option<usize>,
}

#[derive(Debug)]
pub struct BoundaryBlock {
    pub name: String,
    pub exposure: Option<String>,
    pub specification_path: Option<String>,
    pub specification_type: Option<String>,
    pub line: Option<usize>,
}

#[derive(Debug)]
pub struct ComponentBlock {
    pub name: String,
    pub technology: Option<String>,
    pub provides: Vec<String>,
    pub requires: Vec<String>,
    pub line: Option<usize>,
}

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
