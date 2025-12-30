use super::*;

/// Utility functions for internal use.
impl Directory {
    /// Creates the directory on the file system if it does not exist.
    /// Panics if the directory cannot be created.
    /// TODO: revisit name
    pub(super) fn ensure_exists(&self) {
        let path = self.path();
        std::fs::create_dir_all(&path)
            .unwrap_or_else(|e| panic!("Failed to create directory at {}: {e}", path.display()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn ensure_exists() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let directory = Directory {
            base_path: temp_dir.path().to_path_buf(),
            subdirs: vec!["test_dir".to_string()],
        };
        directory.ensure_exists();
        let path = directory.path();

        assert!(path.exists());
        assert!(path.is_dir());
        assert_eq!(path, dir_path);
    }
}
