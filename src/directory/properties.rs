use super::*;

/// Methods to query properties of Directory instances.
impl Directory {
    /// Returns true if the directory exists in the filesystem.
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Returns true if the directory exists and is a directory in the filesystem.
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    /// Returns true if the directory exists and is a file in the filesystem.
    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    /// Returns the path of the directory as a `PathBuf`.
    /// Will return an empty `PathBuf` if the path has an error.
    pub fn path(&self) -> PathBuf {
        self.path.to_path_buf()
    }

    /// Returns true if the directory is persistent
    /// (i.e., has no subdirectories that would be dropped).
    pub fn is_persistent(&self) -> bool {
        self.path.is_persistent()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn exists() {
        let temp_dir = tempdir().unwrap();
        let existing_dir_path = temp_dir.path().join("test_dir_existing");
        let non_existing_dir_path = temp_dir.path().join("test_dir_non_existing");
        let existing_file_path = temp_dir.path().join("test_file.txt");
        std::fs::create_dir_all(&existing_dir_path).unwrap();
        std::fs::write(&existing_file_path, b"Test content").unwrap();

        let existing_dir = Directory::new(&existing_dir_path);
        let non_existing_dir = Directory::new(&non_existing_dir_path);
        let existing_file_as_dir = Directory::new(&existing_file_path);

        assert!(existing_dir.exists());
        assert!(!non_existing_dir.exists());
        assert!(existing_file_as_dir.exists());
    }

    #[test]
    fn is_dir() {
        let temp_dir = tempdir().unwrap();
        let existing_dir_path = temp_dir.path().join("test_dir_existing");
        let non_existing_dir_path = temp_dir.path().join("test_dir_non_existing");
        let existing_file_path = temp_dir.path().join("test_file.txt");
        std::fs::create_dir_all(&existing_dir_path).unwrap();
        std::fs::write(&existing_file_path, b"Test content").unwrap();

        let existing_dir = Directory::new(&existing_dir_path);
        let non_existing_dir = Directory::new(&non_existing_dir_path);
        let existing_file_as_dir = Directory::new(&existing_file_path);

        assert!(existing_dir.is_dir());
        assert!(!non_existing_dir.is_dir());
        assert!(!existing_file_as_dir.is_dir());
    }

    #[test]
    fn is_file() {
        let temp_dir = tempdir().unwrap();
        let existing_dir_path = temp_dir.path().join("test_dir_existing");
        let non_existing_dir_path = temp_dir.path().join("test_dir_non_existing");
        let existing_file_path = temp_dir.path().join("test_file.txt");
        std::fs::create_dir_all(&existing_dir_path).unwrap();
        std::fs::write(&existing_file_path, b"Test content").unwrap();

        let existing_dir = Directory::new(&existing_dir_path);
        let non_existing_dir = Directory::new(&non_existing_dir_path);
        let existing_file_as_dir = Directory::new(&existing_file_path);

        assert!(!existing_dir.is_file());
        assert!(!non_existing_dir.is_file());
        assert!(existing_file_as_dir.is_file());
    }

    #[test]
    fn path_valid() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let directory = Directory::new(&dir_path);

        assert_eq!(directory.path(), dir_path);
    }

    #[test]
    fn path_invalid() {
        let temp_dir = tempdir().unwrap();
        let invalid_path = temp_dir.path().join("..");

        let directory = Directory::new(&invalid_path);
        assert_eq!(directory.path(), invalid_path);
    }

    #[test]
    fn is_persistent() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let persistent_directory = Directory::new(&dir_path).keep();
        let non_persistent_directory = Directory::new(&dir_path);

        assert!(persistent_directory.is_persistent());
        assert!(!non_persistent_directory.is_persistent());
    }
}
