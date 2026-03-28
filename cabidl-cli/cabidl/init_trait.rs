use std::fmt;
use std::path::Path;

/// An error that occurs during project initialization.
#[derive(Debug)]
pub struct InitError {
    pub message: String,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// A template entry from the compile-time index.
#[derive(Debug, Clone)]
pub struct TemplateEntry {
    pub name: String,
    pub language: String,
    pub description: String,
}

/// The Init boundary trait.
///
/// Handles project scaffolding from embedded templates. The template index
/// is generated at compile time from `template.yaml` files. The templates
/// archive is compressed and embedded in the binary via `include_bytes!`.
pub trait Init {
    /// Returns all available templates from the compile-time index.
    /// No decompression needed — the index is built at compile time.
    fn list_templates(&self) -> Vec<TemplateEntry>;

    /// Scaffold a project from a template.
    ///
    /// Decompresses the embedded templates archive to a platform-specific
    /// temp directory, copies the selected template's contents (excluding
    /// `template.yaml`) to `target_dir`, then cleans up the temp directory.
    fn scaffold(
        &self,
        template_name: &str,
        target_dir: &Path,
    ) -> Result<(), InitError>;
}
