use std::io::Read;
use std::path::{Path, PathBuf};

use cabidl_filesystem::Filesystem;
use cabidl_init::{Init, InitError, TemplateEntry};

include!(concat!(env!("OUT_DIR"), "/template_index.rs"));

const TEMPLATES_ARCHIVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/templates.tar.gz"));

pub struct InitImpl {
    fs: Box<dyn Filesystem>,
}

impl InitImpl {
    pub fn new(fs: Box<dyn Filesystem>) -> Self {
        Self { fs }
    }
}

impl Init for InitImpl {
    fn list_templates(&self) -> Vec<TemplateEntry> {
        template_index()
    }

    fn scaffold(
        &self,
        template_name: &str,
        target_dir: &Path,
    ) -> Result<(), InitError> {
        if !template_index().iter().any(|t| t.name == template_name) {
            return Err(InitError {
                message: format!("Unknown template: '{}'", template_name),
            });
        }

        let decoder = flate2::read::GzDecoder::new(TEMPLATES_ARCHIVE);
        let mut archive = tar::Archive::new(decoder);
        let prefix = PathBuf::from(template_name);

        for entry in archive.entries().map_err(|e| InitError {
            message: format!("Failed to read templates archive: {}", e),
        })? {
            let mut entry = entry.map_err(|e| InitError {
                message: format!("Failed to read archive entry: {}", e),
            })?;

            let entry_path = entry
                .path()
                .map_err(|e| InitError {
                    message: format!("Invalid entry path: {}", e),
                })?
                .into_owned();

            let rel = match entry_path.strip_prefix(&prefix) {
                Ok(r) => r.to_path_buf(),
                Err(_) => continue,
            };

            if rel.as_os_str().is_empty() || rel == Path::new("template.yaml") {
                continue;
            }

            let dest = target_dir.join(&rel);

            if entry.header().entry_type().is_dir() {
                self.fs.create_dir_all(&dest).map_err(|e| InitError {
                    message: format!("Failed to create dir '{}': {}", dest.display(), e),
                })?;
            } else {
                if let Some(parent) = dest.parent() {
                    self.fs.create_dir_all(parent).map_err(|e| InitError {
                        message: format!("Failed to create dir '{}': {}", parent.display(), e),
                    })?;
                }
                let mut content = String::new();
                entry.read_to_string(&mut content).map_err(|e| InitError {
                    message: format!("Failed to read '{}': {}", entry_path.display(), e),
                })?;
                self.fs.write_string(&dest, &content).map_err(|e| InitError {
                    message: format!("Failed to write '{}': {}", dest.display(), e),
                })?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cabidl_filesystem_impl::InMemoryFilesystem;
    use cabidl_init::Init;

    #[test]
    fn list_templates_returns_index() {
        let init: Box<dyn Init> = Box::new(InitImpl::new(Box::new(InMemoryFilesystem::new())));
        let templates = init.list_templates();
        assert!(!templates.is_empty());
        assert!(templates.iter().any(|t| t.name == "simple-ls"));
    }

    #[test]
    fn scaffold_unknown_template_returns_error() {
        let init: Box<dyn Init> = Box::new(InitImpl::new(Box::new(InMemoryFilesystem::new())));
        let result = init.scaffold("nonexistent", Path::new("/test"));
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Unknown template"));
    }

    #[test]
    fn scaffold_copies_template_files() {
        let init: Box<dyn Init> = Box::new(InitImpl::new(Box::new(InMemoryFilesystem::new())));
        let result = init.scaffold("simple-ls", Path::new("/test/project"));
        assert!(result.is_ok());
    }
}
