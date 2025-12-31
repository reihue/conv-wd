use super::*;

use crate::Error;

/// Utility functions for internal use.
impl Directory {
    /// Creates the directory on the file system if it does not exist.
    /// TODO: revisit name
    pub(super) fn ensure_exists(&self) -> Result<(), Error> {
        let path = self.path();
        match std::fs::create_dir_all(&path) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::directory_creation_error(path)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn ensure_exists() -> Result<(), Error> {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let directory = Directory {
            base_path: temp_dir.path().to_path_buf(),
            subdirs: vec!["test_dir".to_string()],
        };
        directory.ensure_exists()?;
        let path = directory.path();

        assert!(path.exists());
        assert!(path.is_dir());
        assert_eq!(path, dir_path);

        Ok(())
    }
}
