/// Defines custom error types for the crate.
#[derive(Debug)]
pub enum Error {
    /// Error indicating that a path expected to be a directory is actually a file.
    PathIsNotADirectory(std::path::PathBuf),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PathIsNotADirectory(path) => {
                write!(f, "The path '{}' is not a directory.", path.display())
            }
        }
    }
}

impl std::error::Error for Error {}
