use super::*;

use std::path::{Path, PathBuf};

/// Accessor methods.
impl Directory {
    /// Returns the path of the directory.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the path of the directory as a `PathBuf`.
    pub fn path_buf(&self) -> PathBuf {
        self.path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn path() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let directory = Directory::create(&dir_path);

        assert_eq!(directory.path(), dir_path.as_path());
    }

    #[test]
    fn path_buf() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let directory = Directory::create(&dir_path);

        assert_eq!(directory.path_buf(), dir_path);
    }
}
