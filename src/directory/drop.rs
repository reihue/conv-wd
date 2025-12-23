use super::*;

impl Drop for Directory {
    /// Drops the Directory instance.
    /// If the directory is marked as temporary, it is removed from the file system.
    fn drop(&mut self) {
        if !self.keep_on_drop {
            self.remove();
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
            let directory = Directory {
                path: dir_path.clone(),
                keep_on_drop: false,
            };
            directory.ensure_exists();
        }
        assert!(!dir_path.exists());
    }

    #[test]
    fn drop_persistent_directory() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("persistent_dir");

        {
            let directory = Directory {
                path: dir_path.clone(),
                keep_on_drop: true,
            };
            directory.ensure_exists();
        }

        assert!(dir_path.exists());
        assert!(dir_path.is_dir());
    }
}
