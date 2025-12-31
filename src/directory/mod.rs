use std::path::PathBuf;

/// Represents a directory in the file system.
/// TODO: Add more documentation on provided functionality.
///
/// # Lifecycle:
/// - Internally stores a base path and a relative path of extra subdirectories.
/// - On drop, removes all extra subdirectories unless they are not empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directory {
    /// A base path that will be kept on drop.
    base_path: PathBuf,
    /// The subdirectories that were created when instantiating this struct.
    subdirs: Vec<String>,
}

mod access;
mod cargo;
mod constructors;
mod drop;
mod files;
mod util;

// TODO: add more tests
// - new_subdir
// - new_persistent
// - more complex paths
// - more complex drop behaviour (e.g. non-empty created directories,
//.  multiple `Directory` instances with common ancestors)
