use super::*;

use std::path::Path;

use crate::Error;

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
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref().to_path_buf();
        if path.exists() {
            if !path.is_dir() {
                return Err(Error::PathIsNotADirectory(path));
            }
            return Self::new_persistent(path);
        }

        let dirname = path.file_name().ok_or(Error::malformed_path(&path))?;
        let parent = path.parent().ok_or(Error::malformed_path(&path))?;

        Self::new(parent).and_then(|dir| dir.new_subdir(dirname.to_string_lossy()))
    }

    /// Creates a new persistent `Directory` instance.
    /// I.e. the directory will not be removed from the
    /// file system when the instance is dropped.
    /// Creates the directory on the file system if it does not exist.
    /// TODO: handle errors if the directory cannot be created.
    pub fn new_persistent<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let dir = Self {
            base_path: path.as_ref().to_path_buf(),
            subdirs: Vec::new(),
        };
        dir.ensure_exists()?;
        Ok(dir)
    }

    /// Creates a new `Directory` instance from `self` and a subdirectory name.
    /// If the target path already exists, it is used as the base path.
    /// Otherwise, adds the subdirectory to the internal record of created subdirectories.
    /// Creates the subdirectory on the file system if it does not exist.
    /// TODO: handle directory creation errors
    pub fn new_subdir<S: Into<String>>(mut self, subdir: S) -> Result<Self, Error> {
        let subdir = subdir.into();
        let target_path = self.base_path.join(&subdir);
        if target_path.exists() {
            if !target_path.is_dir() {
                return Err(Error::PathIsNotADirectory(target_path));
            }
            return Self::new_persistent(target_path);
        }

        self.subdirs.push(subdir);
        self.ensure_exists()?;
        Ok(self)
    }

    /// Turns `self` into a persistent directory.
    /// I.e. deletes the information about created subdirectories, so that the
    /// directory will not be removed from the file system when the instance is dropped.
    pub fn keep(mut self) -> Result<Self, Error> {
        for d in &self.subdirs {
            self.base_path.push(d);
        }
        self.subdirs.clear();
        Ok(self)
    }

    /// Creates a new `Directory` instance from self.
    /// Removes all content if the directory already exists.
    pub fn clean(self) -> Result<Self, Error> {
        for entry in std::fs::read_dir(self.path()).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                std::fs::remove_dir_all(&path).unwrap();
            } else {
                std::fs::remove_file(&path).unwrap();
            }
        }
        self.ensure_exists()?;
        Ok(self)
    }

    /// Creates a new `Directory` instance from self.
    /// Adds a `.gitignore` file that causes all content to be ignored by Git.
    pub fn with_gitignore(self) -> Result<Self, Error> {
        self.write_gitignore()?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

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
    fn new_non_existing() -> Result<(), Error> {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        {
            let directory = Directory::new(&dir_path)?;
            let path = directory.path();

            assert!(path.exists());
            assert!(path.is_dir());
            assert_eq!(path, dir_path);
        }
        assert!(!dir_path.exists());

        Ok(())
    }

    #[test]
    fn new_existing() -> Result<(), Error> {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        std::fs::create_dir_all(&dir_path).unwrap();
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        {
            let directory = Directory::new(&dir_path)?;
            let path = directory.path();

            assert!(path.exists());
            assert!(path.is_dir());
            assert_eq!(path, dir_path);
        }
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        Ok(())
    }

    #[test]
    fn new_existing_file_error() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        std::fs::write(&file_path, b"Test content").unwrap();
        assert!(file_path.exists());
        assert!(file_path.is_file());

        let result = Directory::new(&file_path);
        assert_eq!(result, Err(Error::path_is_not_a_directory(file_path)));
    }

    #[test]
    fn keep() -> Result<(), Error> {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("persistent_dir");
        {
            let directory = Directory::new(&dir_path)?.keep()?;

            assert!(directory.base_path.exists());
            assert!(directory.base_path.is_dir());
            assert_eq!(directory.base_path, dir_path);
        }
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        Ok(())
    }

    #[test]
    fn clean() -> Result<(), Error> {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("temp_dir");
        std::fs::create_dir_all(&dir_path).unwrap();
        std::fs::write(dir_path.join("temp_file.txt"), b"Temporary content").unwrap();
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        let directory = Directory::new(&dir_path)?.clean()?;
        let path = directory.path();

        assert!(path.exists());
        assert!(path.is_dir());
        assert_eq!(path, dir_path);
        assert!(std::fs::read_dir(&path).unwrap().next().is_none());

        Ok(())
    }

    #[test]
    fn with_gitignore() -> Result<(), Error> {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("temp_dir");

        let directory = Directory::new(&dir_path)?.with_gitignore()?;
        let path = directory.path();

        assert!(path.exists());
        assert!(path.is_dir());
        assert_eq!(path, dir_path);
        assert!(path.join(".gitignore").exists());
        assert_eq!(
            std::fs::read_to_string(path.join(".gitignore")).unwrap(),
            "*\n"
        );

        Ok(())
    }
}
