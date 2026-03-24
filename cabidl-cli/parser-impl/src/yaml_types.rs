use serde::Deserialize;

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
