use std::io;
use std::path::{Path, PathBuf};

use cabidl_filesystem::Filesystem;

pub struct RealFilesystem;

impl Filesystem for RealFilesystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }

    fn resolve_path(&self, base: &Path, relative: &Path) -> io::Result<PathBuf> {
        let base_dir = if base.is_file() {
            base.parent().unwrap_or(Path::new("."))
        } else {
            base
        };
        base_dir.join(relative).canonicalize()
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        path.canonicalize()
    }

    fn write_string(&self, path: &Path, content: &str) -> io::Result<()> {
        std::fs::write(path, content)
    }
}
