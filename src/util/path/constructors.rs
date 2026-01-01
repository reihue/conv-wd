use super::*;

use crate::util::closest_ancestor;

/// Constructors for `Path`.
impl Path {
    /// Creates a new `Path` instance where the given path is used as the base path.
    pub fn new_persistent<P: AsRef<std::path::Path>>(path: P) -> Self {
        Self {
            base_path: path.as_ref().to_path_buf(),
            subdirs: Vec::new(),
            error: None,
        }
    }

    /// Creates a new `Path` instance.
    /// The given path is analyzed to find the closest existing ancestor.
    /// The base path is set to that ancestor, and any subdirectories
    /// from that ancestor to the original path are stored in `subdirs`.
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        match closest_ancestor(&path) {
            Ok((base_path, subdirs)) => Self {
                base_path,
                subdirs,
                error: None,
            },
            Err(e) => Self {
                base_path: std::path::PathBuf::new(),
                subdirs: Vec::new(),
                error: Some(e),
            },
        }
    }

    /// Transforms the `Path` instance into one where the complete path
    /// is used as the base path, and no subdirectories are stored.
    pub fn keep(mut self) -> Self {
        self.make_persistent();
        self
    }

    /// Adds a subdirectory to the path.
    /// If the target path already exists, the path is transformed into a persistent one.
    /// Otherwise, the subdirectory is added to the internal record of subdirectories.
    pub fn with_subdir<S: Into<String>>(mut self, subdir: S) -> Self {
        self.add_subdir(subdir);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::Error;

    #[test]
    fn new_persistent() {
        let path = Path::new_persistent("some/base/path");
        let expected_path = Path {
            base_path: std::path::PathBuf::from("some/base/path"),
            subdirs: Vec::new(),
            error: None,
        };

        assert_eq!(path, expected_path);
    }

    #[test]
    fn new_existing_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let existing_path = temp_dir.path().join("existing_dir");
        std::fs::create_dir_all(&existing_path).unwrap();

        let path = Path::new(&existing_path);
        let expected_path = Path {
            base_path: existing_path,
            subdirs: Vec::new(),
            error: None,
        };

        assert_eq!(path, expected_path);
    }

    #[test]
    fn new_existing_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let existing_file_path = temp_dir.path().join("existing_file");
        std::fs::write(&existing_file_path, "test").unwrap();

        let path = Path::new(&existing_file_path);
        let expected_path = Path {
            base_path: existing_file_path,
            subdirs: Vec::new(),
            error: None,
        };

        assert_eq!(path, expected_path);
    }

    #[test]
    fn new_existing_parent() {
        let temp_dir = tempfile::tempdir().unwrap();
        let parent_path = temp_dir.path().join("parent_dir");
        std::fs::create_dir_all(&parent_path).unwrap();
        let child_path = parent_path.join("child_dir");

        let path = Path::new(&child_path);
        let expected_path = Path {
            base_path: parent_path,
            subdirs: vec!["child_dir".to_string()],
            error: None,
        };

        assert_eq!(path, expected_path);
    }

    #[test]
    fn new_non_existing() {
        let path = Path::new("some/base/path");
        let expected_path = Path {
            base_path: std::path::PathBuf::new(),
            subdirs: vec!["some".to_string(), "base".to_string(), "path".to_string()],
            error: None,
        };

        assert_eq!(path, expected_path);
    }

    #[test]
    fn new_empty_path() {
        let path = Path::new("");
        let expected_path = Path {
            base_path: std::path::PathBuf::new(),
            subdirs: Vec::new(),
            error: None,
        };

        assert_eq!(path, expected_path);
    }

    #[test]
    fn new_no_filename() {
        let path = Path::new("some/path/..");
        let expected_path = Path {
            base_path: std::path::PathBuf::new(),
            subdirs: Vec::new(),
            error: Some(Error::malformed_path("some/path/..")),
        };

        assert_eq!(path, expected_path);
    }

    #[test]
    fn keep() {
        let original_path = Path {
            base_path: std::path::PathBuf::from("base/path"),
            subdirs: vec!["subdir1".to_string(), "subdir2".to_string()],
            error: None,
        };

        let kept_path = original_path.keep();
        let expected_path = Path {
            base_path: std::path::PathBuf::from("base/path/subdir1/subdir2"),
            subdirs: Vec::new(),
            error: None,
        };

        assert_eq!(kept_path, expected_path);
    }

    #[test]
    fn with_subdir_non_existing() {
        let base_path = Path {
            base_path: std::path::PathBuf::from("base/path"),
            subdirs: Vec::new(),
            error: None,
        };

        let new_path = base_path.with_subdir("subdir1");
        let expected_path = Path {
            base_path: std::path::PathBuf::from("base/path"),
            subdirs: vec!["subdir1".to_string()],
            error: None,
        };

        assert_eq!(new_path, expected_path);
    }

    #[test]
    fn with_subdir_existing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let existing_path = temp_dir.path().join("existing_dir");
        std::fs::create_dir_all(&existing_path).unwrap();

        let base_path = Path {
            base_path: temp_dir.path().to_path_buf(),
            subdirs: Vec::new(),
            error: None,
        };

        let new_path = base_path.with_subdir("existing_dir");
        let expected_path = Path {
            base_path: existing_path,
            subdirs: Vec::new(),
            error: None,
        };

        assert_eq!(new_path, expected_path);
    }
}
