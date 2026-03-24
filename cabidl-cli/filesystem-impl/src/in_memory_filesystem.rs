use std::collections::HashMap;
use std::io;
use std::path::{Component, Path, PathBuf};

use cabidl_filesystem::Filesystem;

/// In-memory filesystem for testing. Maps paths to file contents.
pub struct InMemoryFilesystem {
    files: HashMap<PathBuf, String>,
}

impl InMemoryFilesystem {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn add_file<P: Into<PathBuf>>(&mut self, path: P, content: &str) {
        self.files.insert(path.into(), content.to_string());
    }
}

impl Filesystem for InMemoryFilesystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File not found: {}", path.display()),
                )
            })
    }

    fn resolve_path(&self, base: &Path, relative: &Path) -> io::Result<PathBuf> {
        let base_dir = if self.files.contains_key(base) {
            base.parent().unwrap_or(Path::new("/"))
        } else {
            base
        };
        Ok(normalize_path(&base_dir.join(relative)))
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        Ok(normalize_path(path))
    }
}

/// Normalize a path by resolving `.` and `..` components without OS calls.
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                components.pop();
            }
            Component::CurDir => {}
            c => components.push(c),
        }
    }
    components.iter().collect()
}
