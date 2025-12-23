use super::*;

/// Utility functions for internal use.
impl Directory {
    /// Creates the directory on the file system if it does not exist.
    /// Panics if the directory cannot be created.
    pub(super) fn ensure_exists(&self) {
        std::fs::create_dir_all(&self.path).unwrap_or_else(|e| {
            panic!("Failed to create directory at {}: {e}", self.path.display())
        });
    }

    /// Removes the directory from the file system if it still exists.
    /// Panics if the directory cannot be removed.
    pub(super) fn remove(&self) {
        if self.path.exists() {
            std::fs::remove_dir_all(&self.path).unwrap_or_else(|e| {
                panic!("Failed to remove directory at {}: {e}", self.path.display())
            });
        }
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
            path: dir_path.clone(),
            keep_on_drop: false,
        };
        directory.ensure_exists();

        assert!(directory.path.exists());
        assert!(directory.path.is_dir());
        assert_eq!(directory.path, dir_path);
    }

    #[test]
    fn remove() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        std::fs::create_dir_all(&dir_path).unwrap();
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        let directory = Directory {
            path: dir_path.clone(),
            keep_on_drop: true,
        };

        directory.remove();

        assert!(!dir_path.exists());
    }
}
