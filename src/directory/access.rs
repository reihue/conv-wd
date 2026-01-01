use super::*;

use std::path::PathBuf;

/// Accessor methods.
impl Directory {
    /// Returns the path of the directory as a `PathBuf`.
    pub fn path(&self) -> PathBuf {
        let mut path = self.base_path.clone();
        for subdir in &self.subdirs {
            path.push(subdir);
        }
        path
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

        let directory = Directory::new(&dir_path);

        assert_eq!(directory.path(), dir_path.as_path());
    }
}
