use super::*;

use std::path::Path;

/// Constructors and factory methods.
impl Directory {
    /// Creates a new Directory instance with the given path.
    /// The directory is also created on the file system if it does not exist.
    /// Panics if the directory cannot be created.
    ///
    /// # Arguments
    /// * `path` - The path where the directory should be created.
    pub fn create<P: AsRef<Path>>(path: P) -> Self {
        let dir = Self {
            path: path.as_ref().to_path_buf(),
            keep_on_drop: false,
        };

        dir.ensure_exists();
        dir
    }

    /// Creates a new persistent Directory instance from self.
    /// The directory will not be removed when the instance is dropped.
    pub fn keep(mut self) -> Self {
        self.keep_on_drop = true;
        self
    }

    /// Creates a new Directory instance from self.
    /// Removes all content on creation.
    pub fn clean(self) -> Self {
        self.remove();
        self.ensure_exists();
        self
    }

    /// Creates a new temporary Directory instance from self.
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
    fn create() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        {
            let directory = Directory::create(&dir_path);

            assert!(directory.path.exists());
            assert!(directory.path.is_dir());
            assert_eq!(directory.path, dir_path);
        }
        assert!(!dir_path.exists());
    }

    #[test]
    fn keep() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("persistent_dir");
        {
            let directory = Directory::create(&dir_path).keep();

            assert!(directory.path.exists());
            assert!(directory.path.is_dir());
            assert_eq!(directory.path, dir_path);
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

        assert!(directory.path.exists());
        assert!(directory.path.is_dir());
        assert_eq!(directory.path, dir_path);
        assert!(std::fs::read_dir(&dir_path).unwrap().next().is_none());
    }

    #[test]
    fn with_gitignore() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("temp_dir");

        let directory = Directory::create(&dir_path).with_gitignore();

        assert!(directory.path.exists());
        assert!(directory.path.is_dir());
        assert_eq!(directory.path, dir_path);
        assert!(dir_path.join(".gitignore").exists());
        assert_eq!(
            std::fs::read_to_string(dir_path.join(".gitignore")).unwrap(),
            "*\n"
        );
    }
}
