use std::io;
use std::path::{Path, PathBuf};

/// The Filesystem boundary trait.
///
/// Provides an abstraction over file I/O so that the parser can be tested
/// with in-memory file systems without touching the real filesystem.
pub trait Filesystem {
    /// Read the entire contents of a file as a UTF-8 string.
    fn read_to_string(&self, path: &Path) -> io::Result<String>;

    /// Resolve a potentially relative path against a base file path.
    /// The base path is the file containing the `<!-- @include -->` directive.
    /// The method resolves the relative path against the base file's parent directory.
    fn resolve_path(&self, base: &Path, relative: &Path) -> io::Result<PathBuf>;

    /// Check whether a file exists at the given path.
    fn exists(&self, path: &Path) -> bool;

    /// Return the canonical, absolute form of a path.
    /// For real filesystems, this resolves symlinks and relative components.
    /// For in-memory filesystems, this normalizes the path without OS calls.
    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf>;

    /// Write the given content to a file as UTF-8, creating or overwriting the file.
    fn write_string(&self, path: &Path, content: &str) -> io::Result<()>;

    /// Create a directory and all its parent directories.
    /// For real filesystems, delegates to `std::fs::create_dir_all`.
    /// For in-memory filesystems, this is a no-op (files are stored by path).
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
}
