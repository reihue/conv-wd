use super::*;

use std::path::Path;

/// Constructors and factory methods.
impl Directory {
    /// Creates a new `Directory` instance.
    ///
    /// Determines which parent directories have to be created and stores this
    /// information internally for initialization and cleanup on drop.
    ///
    /// # Arguments
    /// * `path` - The path where the directory should be created.
    ///
    /// TODO Handle errors if the path is malformed (e.g. has no parent).
    /// - Currently, the function panics in this case.
    /// - Returning a `Result` is not an option, because I want to keep creating
    ///   `Directory` instances as simple as possible.
    /// - Possible solution: Deferr the path (esp. parent) resolution to `initialize()`.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        if path.exists() {
            return Self::new_persistent(path);
        }

        let dirname = path.file_name().expect("Malformed path: no file name");
        let parent = path.parent().expect("Malformed path: no parent");

        Self::new(parent).new_subdir(dirname.to_string_lossy())
    }

    /// Creates a new persistent `Directory` instance.
    ///
    /// I.e. the directory will not be removed from the
    /// file system when the instance is dropped.
    /// TODO: handle errors if the directory cannot be created.
    pub fn new_persistent<P: AsRef<Path>>(path: P) -> Self {
        let dir = Self {
            base_path: path.as_ref().to_path_buf(),
            subdirs: Vec::new(),
            clean_on_init: CLEAN_ON_INIT_DEFAULT,
            gitignore_on_init: GITIGNORE_ON_INIT_DEFAULT,
        };
        dir
    }

    /// Creates a new `Directory` instance from `self` and a subdirectory name.
    ///
    /// If the target path already exists, it is kept persistent.
    /// Otherwise, adds the subdirectory to the internal record of created subdirectories.
    pub fn new_subdir<S: Into<String>>(mut self, subdir: S) -> Self {
        let subdir = subdir.into();
        let target_path = self.base_path.join(&subdir);
        if target_path.exists() {
            return Self::new_persistent(target_path);
        }

        self.subdirs.push(subdir);
        self
    }

    /// Turns `self` into a persistent directory.
    /// I.e. deletes the information about created subdirectories, so that the
    /// directory will not be removed from the file system when the instance is dropped.
    pub fn keep(self) -> Self {
        Self::new_persistent(self.path())
    }

    /// Creates a new `Directory` instance from self.
    /// Records that the directory should be cleaned on initialization.
    pub fn clean(mut self) -> Self {
        self.clean_on_init = true;
        self
    }

    /// Creates a new `Directory` instance from self.
    /// Records that a `.gitignore` file should be created on initialization.
    pub fn with_gitignore(mut self) -> Self {
        self.gitignore_on_init = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    use crate::util::Error;

    // TODO: test cases for new:
    // - regular cases:
    //   - directory already exists
    //   - parent directory exists
    //   - parent directory does not exist
    // - error cases:
    //   - path exists but is a file
    //   - parent path exists but is a file
    //   - path is a single component (no parent)
    //   - path ends in ".." (also no parent)

    #[test]
    fn new_non_existing() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        {
            let directory = Directory::new(&dir_path);
            let path = directory.path();
            assert!(!path.exists());

            assert_eq!(directory.initialize(), Ok(()));
            assert!(path.exists());
            assert!(path.is_dir());
            assert_eq!(path, dir_path);
        }
        assert!(!dir_path.exists());
    }

    #[test]
    fn new_existing() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        std::fs::create_dir_all(&dir_path).unwrap();
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        {
            let directory = Directory::new(&dir_path);
            let path = directory.path();

            assert!(path.exists());
            assert!(path.is_dir());
            assert_eq!(path, dir_path);
        }
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());
    }

    #[test]
    fn new_existing_file_error() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        std::fs::write(&file_path, b"Test content").unwrap();
        assert!(file_path.exists());
        assert!(file_path.is_file());

        let directory = Directory::new(&file_path);
        assert!(file_path.exists());
        assert!(file_path.is_file());
        let result = directory.initialize();
        assert_eq!(result, Err(Error::path_is_not_a_directory(file_path)));
    }

    #[test]
    fn keep() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("persistent_dir");
        {
            let directory = Directory::new(&dir_path).keep();
            assert!(!directory.base_path.exists());

            assert_eq!(directory.initialize(), Ok(()));
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

        let directory = Directory::new(&dir_path).clean();
        assert_eq!(directory.initialize(), Ok(()));
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

        let directory = Directory::new(&dir_path).with_gitignore();
        assert!(!dir_path.exists());
        assert_eq!(directory.initialize(), Ok(()));

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
