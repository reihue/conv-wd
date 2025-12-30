use super::*;

use std::path::Path;

/// Constructors and factory methods.
impl Directory {
    /// Creates a new `Directory` instance.
    ///
    /// # Arguments
    /// * `path` - The path where the directory should be created.
    ///
    /// # Behaviour
    /// - Any required parent directories that do not already exist will be created.
    /// - A record of which subdirectories were created will be stored internally.
    /// - On drop, all created subdirectories will be removed, unless they contain
    ///   any content that was not created as part of this process.
    pub fn create<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        if path.exists() {
            if !path.is_dir() {
                panic!("Path {} exists but is not a directory", path.display());
            }
            return Self::new_persistent(path);
        }

        let (parent, subdir) = if let Some(parent) = path.parent() {
            (
                parent,
                path.file_name().unwrap().to_str().unwrap().to_string(),
            )
        } else {
            panic!(
                "Cannot create directory at {}: no parent directory",
                path.display()
            );
        };

        Self::create(parent).new_subdir(&subdir)
    }

    /// Creates a new persistent `Directory` instance.
    /// I.e. the directory will not be removed from the
    /// file system when the instance is dropped.
    /// Creates the directory on the file system if it does not exist.
    /// TODO: handle errors if the directory cannot be created.
    pub fn new_persistent<P: AsRef<Path>>(path: P) -> Self {
        let dir = Self {
            base_path: path.as_ref().to_path_buf(),
            subdirs: Vec::new(),
        };
        dir.ensure_exists();
        dir
    }

    /// Creates a new `Directory` instance from `self` and a subdirectory name.
    /// If the target path already exists, it is used as the base path.
    /// Otherwise, adds the subdirectory to the internal record of created subdirectories.
    /// Creates the subdirectory on the file system if it does not exist.
    /// TODO: handle directory creation errors
    pub fn new_subdir<S: Into<String>>(mut self, subdir: S) -> Self {
        let subdir = subdir.into();
        let target_path = self.base_path.join(&subdir);
        if target_path.exists() {
            if !target_path.is_dir() {
                panic!(
                    "Path {} exists but is not a directory",
                    target_path.display()
                );
            }
            return Self::new_persistent(target_path);
        }

        self.subdirs.push(subdir);
        self.ensure_exists();
        self
    }

    /// Turns `self` into a persistent directory.
    /// I.e. deletes the information about created subdirectories, so that the
    /// directory will not be removed from the file system when the instance is dropped.
    pub fn keep(mut self) -> Self {
        for d in &self.subdirs {
            self.base_path.push(d);
        }
        self.subdirs.clear();
        self
    }

    /// Creates a new `Directory` instance from self.
    /// Removes all content if the directory already exists.
    pub fn clean(self) -> Self {
        for entry in std::fs::read_dir(self.path()).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                std::fs::remove_dir_all(&path).unwrap();
            } else {
                std::fs::remove_file(&path).unwrap();
            }
        }
        self.ensure_exists();
        self
    }

    /// Creates a new `Directory` instance from self.
    /// Adds a `.gitignore` file that causes all content to be ignored by Git.
    pub fn with_gitignore(self) -> Self {
        self.write_gitignore();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn create_non_existing() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        {
            let directory = Directory::create(&dir_path);
            let path = directory.path();

            assert!(path.exists());
            assert!(path.is_dir());
            assert_eq!(path, dir_path);
        }
        assert!(!dir_path.exists());
    }

    #[test]
    fn create_existing() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        std::fs::create_dir_all(&dir_path).unwrap();
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        {
            let directory = Directory::create(&dir_path);
            let path = directory.path();

            assert!(path.exists());
            assert!(path.is_dir());
            assert_eq!(path, dir_path);
        }
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());
    }

    #[test]
    fn create_existing_file_panics() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        std::fs::write(&file_path, b"Test content").unwrap();
        assert!(file_path.exists());
        assert!(file_path.is_file());

        let result = std::panic::catch_unwind(|| {
            Directory::create(&file_path);
        });
        assert!(result.is_err());
    }

    #[test]
    fn keep() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("persistent_dir");
        {
            let directory = Directory::create(&dir_path).keep();

            assert!(directory.base_path.exists());
            assert!(directory.base_path.is_dir());
            assert_eq!(directory.base_path, dir_path);
        }
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());
    }

    #[test]
    fn clean() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("temp_dir");
        std::fs::create_dir_all(&dir_path).unwrap();
        std::fs::write(dir_path.join("temp_file.txt"), b"Temporary content").unwrap();
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        let directory = Directory::create(&dir_path).clean();
        let path = directory.path();

        assert!(path.exists());
        assert!(path.is_dir());
        assert_eq!(path, dir_path);
        assert!(std::fs::read_dir(&path).unwrap().next().is_none());
    }

    #[test]
    fn with_gitignore() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("temp_dir");

        let directory = Directory::create(&dir_path).with_gitignore();
        let path = directory.path();

        assert!(path.exists());
        assert!(path.is_dir());
        assert_eq!(path, dir_path);
        assert!(path.join(".gitignore").exists());
        assert_eq!(
            std::fs::read_to_string(path.join(".gitignore")).unwrap(),
            "*\n"
        );
    }
}
