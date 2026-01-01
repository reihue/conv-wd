use super::*;

/// Property accessor methods.
impl Path {
    /// Resolves the`Path` to a `PathBuf`.
    /// This combines the base path and all subdirectories into a single path.
    pub fn to_path_buf(&self) -> std::path::PathBuf {
        let mut path = self.base_path.clone();
        for subdir in &self.subdirs {
            path.push(subdir);
        }
        path
    }

    /// Returns true if the `Path` exists in the filesystem.
    pub fn exists(&self) -> bool {
        self.to_path_buf().exists()
    }

    /// Returns true if the `Path` exists and is a directory in the filesystem.
    pub fn is_dir(&self) -> bool {
        self.to_path_buf().is_dir()
    }

    /// Returns true if the `Path` exists and is a file in the filesystem.
    pub fn is_file(&self) -> bool {
        self.to_path_buf().is_file()
    }

    /// Returns true if the `Path` is persistent
    /// (i.e., has no subdirectories that would be dropped).
    pub fn is_persistent(&self) -> bool {
        self.subdirs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_path_buf_persistent() {
        let path = Path {
            base_path: std::path::PathBuf::from("base/path/"),
            subdirs: Vec::new(),
            error: None,
        };

        let expected_path = std::path::PathBuf::from("base/path/");
        assert_eq!(path.to_path_buf(), expected_path);
    }

    #[test]
    fn to_path_buf_temp() {
        let path = Path {
            base_path: std::path::PathBuf::from("base/path/"),
            subdirs: vec!["subdir1".to_string(), "subdir2".to_string()],
            error: None,
        };

        let expected_path = std::path::PathBuf::from("base/path/subdir1/subdir2");
        assert_eq!(path.to_path_buf(), expected_path);
    }

    #[test]
    fn exists_existing_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path().join("existing_dir");
        std::fs::create_dir_all(&dir_path).unwrap();

        let path = Path {
            base_path: dir_path,
            subdirs: Vec::new(),
            error: None,
        };
        assert!(path.exists());
    }

    #[test]
    fn exists_existing_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("existing_file.txt");
        std::fs::write(&file_path, b"Test content").unwrap();

        let path = Path {
            base_path: file_path,
            subdirs: Vec::new(),
            error: None,
        };
        assert!(path.exists());
    }

    #[test]
    fn exists_non_existing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path().join("non_existing_entry");

        let path = Path {
            base_path: dir_path,
            subdirs: Vec::new(),
            error: None,
        };
        assert!(!path.exists());
    }

    #[test]
    fn is_dir_existing_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path().join("existing_dir");
        std::fs::create_dir_all(&dir_path).unwrap();

        let path = Path {
            base_path: dir_path,
            subdirs: Vec::new(),
            error: None,
        };
        assert!(path.is_dir());
    }

    #[test]
    fn is_dir_existing_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("existing_file.txt");
        std::fs::write(&file_path, b"Test content").unwrap();

        let path = Path {
            base_path: file_path,
            subdirs: Vec::new(),
            error: None,
        };
        assert!(!path.is_dir());
    }

    #[test]
    fn is_dir_non_existing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path().join("non_existing_entry");

        let path = Path {
            base_path: dir_path,
            subdirs: Vec::new(),
            error: None,
        };
        assert!(!path.is_dir());
    }

    #[test]
    fn is_file_existing_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("existing_file.txt");
        std::fs::write(&file_path, b"Test content").unwrap();

        let path = Path {
            base_path: file_path,
            subdirs: Vec::new(),
            error: None,
        };

        assert!(path.is_file());
    }

    #[test]
    fn is_file_existing_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path().join("existing_dir");
        std::fs::create_dir_all(&dir_path).unwrap();

        let path = Path {
            base_path: dir_path,
            subdirs: Vec::new(),
            error: None,
        };

        assert!(!path.is_file());
    }

    #[test]
    fn is_file_non_existing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("non_existing_entry");

        let path = Path {
            base_path: file_path,
            subdirs: Vec::new(),
            error: None,
        };

        assert!(!path.is_file());
    }
}
