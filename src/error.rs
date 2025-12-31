/// Defines custom error types for the crate.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Indicates that a path is not a directory, although it was expected to be one.
    PathIsNotADirectory(std::path::PathBuf),
    /// Indicates that a path's parent directory does not exist as expected.
    PathDoesNotExist(std::path::PathBuf),
    /// Indicates that a path is absolute, although it was expected to be relative.
    PathIsAbsolute(std::path::PathBuf),
    /// Indicates a malformed path, e.g., when extracting parent directory or file name fails.
    MalformedPath(std::path::PathBuf),
    /// Indicates an error during a file write operation.
    FileWriteError(std::path::PathBuf),
    /// Indicates an error during directory creation.
    DirectoryCreationError(std::path::PathBuf),
    /// Indicates a JSON error.
    JsonError(String),
    /// Indicates a TOML error.
    TomlError(String),
}

// TODO: revisit error messages for clarity and completeness
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            PathIsNotADirectory(path) => {
                write!(f, "The path '{}' is not a directory.", path.display())
            }
            PathDoesNotExist(path) => {
                write!(f, "The path '{}' does not exist.", path.display())
            }
            PathIsAbsolute(path) => {
                write!(
                    f,
                    "The path '{}' is absolute, but a relative path was expected.",
                    path.display()
                )
            }
            MalformedPath(path) => {
                write!(f, "The path '{}' is malformed.", path.display())
            }
            DirectoryCreationError(path) => {
                write!(
                    f,
                    "Failed to create directory at path '{}'.",
                    path.display()
                )
            }
            FileWriteError(path) => {
                write!(f, "Failed to write to file at path '{}'.", path.display())
            }
            JsonError(err) => {
                write!(f, "{}", err)
            }
            TomlError(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl Error {
    /// Creates a `PathIsNotADirectory` error for the given path.
    pub fn path_is_not_a_directory<P: AsRef<std::path::Path>>(path: P) -> Self {
        Error::PathIsNotADirectory(path.as_ref().to_path_buf())
    }

    /// Creates a `ParentDirectoryDoesNotExist` error for the given path.
    pub fn parent_directory_does_not_exist<P: AsRef<std::path::Path>>(path: P) -> Self {
        Error::PathDoesNotExist(path.as_ref().to_path_buf())
    }

    /// Creates a `PathIsAbsolute` error for the given path.
    pub fn path_is_absolute<P: AsRef<std::path::Path>>(path: P) -> Self {
        Error::PathIsAbsolute(path.as_ref().to_path_buf())
    }

    /// Creates a `MalformedPath` error for the given path.
    pub fn malformed_path<P: AsRef<std::path::Path>>(path: P) -> Self {
        Error::MalformedPath(path.as_ref().to_path_buf())
    }

    /// Creates a `FileWriteError` for the given path.
    pub fn file_write_error<P: AsRef<std::path::Path>>(path: P) -> Self {
        Error::FileWriteError(path.as_ref().to_path_buf())
    }
    /// Creates a `DirectoryCreationError` for the given path.
    pub fn directory_creation_error<P: AsRef<std::path::Path>>(path: P) -> Self {
        Error::DirectoryCreationError(path.as_ref().to_path_buf())
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::TomlError(err.to_string())
    }
}
