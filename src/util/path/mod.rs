use crate::util::Error;

/// Reperesents a filesystem path.
/// Consists of a base path and optional subdirectories that are
/// removed when the Path instance is dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    /// A base path that will be kept on drop.
    base_path: std::path::PathBuf,
    /// The subdirectories that were created when instantiating this struct.
    subdirs: Vec<String>,
    /// Error information if the path could not be created.
    pub error: Option<Error>,
}

mod constructors;
mod modifiers;
mod properties;
mod util;
