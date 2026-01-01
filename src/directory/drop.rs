use super::*;

impl Drop for Directory {
    /// Drops the Directory instance.
    /// If the directory is marked as temporary, it is removed from the file system.
    /// TODO: Improve error handling, differentiate between non-empty and other errors?
    fn drop(&mut self) {
        let mut path = self.path();
        while path != self.base_path && std::fs::remove_dir(&path).is_ok() {
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn drop_temporary_directory() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("temp_dir");

        {
            let directory = Directory::new(temp_dir.path()).new_subdir("temp_dir");
            assert_eq!(directory.initialize(), Ok(()));
            assert!(dir_path.exists());
            assert!(dir_path.is_dir());
        }
        assert!(!dir_path.exists());
    }

    #[test]
    fn drop_persistent_directory() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("persistent_dir");

        {
            let directory = Directory::new_persistent(&dir_path);
            assert_eq!(directory.initialize(), Ok(()));
        }

        assert!(dir_path.exists());
        assert!(dir_path.is_dir());
    }
}
