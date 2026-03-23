use std::io;
use std::path::Path;

/// The Filesystem boundary trait.
///
/// Provides an abstraction over file I/O so that the parser can be tested
/// with in-memory file systems without touching the real filesystem.
pub trait Filesystem {
    /// Read the entire contents of a file as a UTF-8 string.
    fn read_to_string(&self, path: &Path) -> io::Result<String>;

    /// Resolve a potentially relative path against a base directory.
    /// Used when processing `<!-- @include -->` directives.
    fn resolve_path(&self, base: &Path, relative: &Path) -> io::Result<std::path::PathBuf>;

    /// Check whether a file exists at the given path.
    fn exists(&self, path: &Path) -> bool;
}
