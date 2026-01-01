use super::*;

/// Modifiers for `Path`.
impl Path {
    /// Makes the path persistent, i.e., removes all subdirectories
    /// from the internal record and uses the full path as the base path.
    pub fn make_persistent(&mut self) {
        self.base_path = self.to_path_buf();
        self.subdirs.clear();
    }

    /// Adds a subdirectory to the path.
    /// If the target path already exists, the path is transformed into a persistent one.
    /// Otherwise, the subdirectory is added to the internal record of subdirectories.
    pub fn add_subdir<S: Into<String>>(&mut self, subdir: S) {
        let subdir = subdir.into();
        let target_path = self.to_path_buf().join(&subdir);
        if target_path.exists() {
            self.base_path = target_path;
            self.subdirs.clear();
            return;
        }

        self.subdirs.push(subdir);
    }

    /// Tries to remove all recorded subdirectories of the base path from the filesystem.
    /// Relies on `std::fs::remove_dir` to only remove empty directories.
    ///
    /// Notes:
    /// - This operation may also fail for other reasons (e.g., permissions).
    /// - If a directory is not empty, the dropping process stops there, leaving
    ///   any remaining subdirectories and the base path intact.
    ///   In particular, this means that if any files were created in the subdirectories,
    ///   or if e.g. multiple `Path` instances share subdirectories, those directories
    ///   will not be removed.
    /// TODO:
    /// - Avoid deleting important directories by mistake (e.g. root, user home, also through symlinks)?
    pub fn remove(&mut self) {
        while !self.subdirs.is_empty() && std::fs::remove_dir(self.to_path_buf()).is_ok() {
            self.subdirs.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn make_persistent() {
        let mut path = Path {
            base_path: std::path::PathBuf::from("base/path/"),
            subdirs: vec!["subdir1".to_string(), "subdir2".to_string()],
            error: None,
        };

        path.make_persistent();

        let expected_path = Path {
            base_path: std::path::PathBuf::from("base/path/subdir1/subdir2"),
            subdirs: Vec::new(),
            error: None,
        };

        assert_eq!(path, expected_path);
    }

    #[test]
    fn add_subdir_non_existing() {
        let mut path = Path {
            base_path: std::path::PathBuf::from("base/path/"),
            subdirs: Vec::new(),
            error: None,
        };

        path.add_subdir("subdir1");

        let expected_path = Path {
            base_path: std::path::PathBuf::from("base/path/"),
            subdirs: vec!["subdir1".to_string()],
            error: None,
        };

        assert_eq!(path, expected_path);
    }

    #[test]
    fn add_subdir_existing() {
        let temp_dir = tempdir().unwrap();
        let existing_path = temp_dir.path().join("existing_dir");
        std::fs::create_dir_all(&existing_path).unwrap();

        let mut path = Path {
            base_path: temp_dir.path().to_path_buf(),
            subdirs: vec![],
            error: None,
        };

        path.add_subdir("existing_dir");
        let expected_path = Path {
            base_path: existing_path,
            subdirs: Vec::new(),
            error: None,
        };

        assert_eq!(path, expected_path);
    }

    #[test]
    fn remove_path_with_recorded_subdirs() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().join("base_path");
        let subdir1 = base_path.join("subdir1");
        let subdir2 = subdir1.join("subdir2");

        std::fs::create_dir_all(&subdir2).unwrap();

        let mut path = Path {
            base_path: base_path.clone(),
            subdirs: vec!["subdir1".to_string(), "subdir2".to_string()],
            error: None,
        };

        assert!(subdir2.exists());
        assert!(subdir2.is_dir());

        path.remove();

        assert!(!subdir2.exists());
        assert!(!subdir1.exists());
        assert!(base_path.exists());
        assert!(base_path.is_dir());
    }

    #[test]
    fn remove_path_without_recorded_subdirs() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().join("base_path");
        std::fs::create_dir_all(&base_path).unwrap();

        let mut path = Path {
            base_path: base_path.clone(),
            subdirs: Vec::new(),
            error: None,
        };

        assert!(base_path.exists());
        assert!(base_path.is_dir());

        path.remove();

        assert!(base_path.exists());
        assert!(base_path.is_dir());
    }

    #[test]
    fn remove_nonempty_path() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().join("base_path");
        let subdir = base_path.join("subdir");
        let file_in_subdir = subdir.join("file.txt");

        std::fs::create_dir_all(&subdir).unwrap();
        std::fs::write(&file_in_subdir, b"Test content").unwrap();

        let mut path = Path {
            base_path: base_path.clone(),
            subdirs: vec!["subdir".to_string()],
            error: None,
        };

        assert!(file_in_subdir.exists());
        assert!(file_in_subdir.is_file());

        path.remove();

        assert!(file_in_subdir.exists());
    }

    #[test]
    fn remove_shared_recorded_subdirs() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().join("base_path");
        let subdir = base_path.join("shared_subdir");
        let p1_dir = subdir.join("p1_dir");
        let p2_dir = subdir.join("p2_dir");

        std::fs::create_dir_all(&p1_dir).unwrap();
        std::fs::create_dir_all(&p2_dir).unwrap();

        let mut path1 = Path {
            base_path: base_path.clone(),
            subdirs: vec!["shared_subdir".to_string(), "p1_dir".to_string()],
            error: None,
        };

        let mut path2 = Path {
            base_path: base_path.clone(),
            subdirs: vec!["shared_subdir".to_string(), "p2_dir".to_string()],
            error: None,
        };

        // Both directories should exist while both Path instances are alive.
        assert!(p1_dir.exists());
        assert!(p1_dir.is_dir());
        assert!(p2_dir.exists());
        assert!(p2_dir.is_dir());

        path2.remove();

        // p2_dir should be removed, but p1_dir should still exist.
        assert!(p1_dir.exists());
        assert!(p1_dir.is_dir());
        assert!(!p2_dir.exists());

        path1.remove();

        // Both directories and the subdir should be removed now.
        // The base path should still exist.
        assert!(!p1_dir.exists());
        assert!(!p2_dir.exists());
        assert!(!subdir.exists());
        assert!(base_path.exists());
        assert!(base_path.is_dir());
    }
}
