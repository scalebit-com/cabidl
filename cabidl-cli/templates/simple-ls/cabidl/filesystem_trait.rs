use std::io;
use std::path::Path;
use std::time::SystemTime;

/// Represents a single directory entry.
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
}

/// File metadata returned for long-format display.
pub struct Metadata {
    pub size: u64,
    pub permissions: u32,
    pub modified: SystemTime,
    pub is_dir: bool,
}

/// The Filesystem boundary trait.
///
/// Provides an abstraction over filesystem operations needed by the CLI.
/// Implementations read directory contents and file metadata from the OS.
pub trait Filesystem {
    /// List all entries in the given directory.
    fn list_entries(&self, path: &Path) -> io::Result<Vec<DirEntry>>;

    /// Get metadata for a specific file or directory.
    fn get_metadata(&self, path: &Path) -> io::Result<Metadata>;
}
