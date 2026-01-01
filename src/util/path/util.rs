use super::*;

use crate::util::{Error, Result};

/// Utility functions.
/// TODO: Revisit name and location? Is "util" appropriate here any more?
impl Path {
    /// Creates the directory on the file system if it does not exist.
    /// Also performs cleanup if and creates a `.gitignore` file
    /// if those options are enabled.
    pub fn initialize(&self) -> Result<()> {
        self.ensure_exists()?;
        self.verify_is_directory()?;
        Ok(())
    }

    /// Creates the directory on the file system if it does not exist.
    pub fn ensure_exists(&self) -> Result<()> {
        let path = self.to_path_buf();
        if !path.exists() {
            std::fs::create_dir_all(&path).map_err(|_| Error::directory_creation_error(path))?;
        }
        Ok(())
    }

    /// Returns an error if `self`'s path does not exist.
    pub fn verify_exists(&self) -> Result<()> {
        let path = self.to_path_buf();
        if !path.exists() {
            return Err(Error::path_does_not_exist(path));
        }
        Ok(())
    }

    /// Returns an error if `self`'s path is not a directory.
    pub fn verify_is_directory(&self) -> Result<()> {
        let path = self.to_path_buf();
        if !path.is_dir() {
            return Err(Error::path_is_not_a_directory(path));
        }
        Ok(())
    }

    /// Removes all content of the directory.
    /// Returns an error if the directory does not exist or is not a directory.
    pub fn remove_contents(&self) -> Result<()> {
        let path = self.to_path_buf();
        self.verify_exists()?;
        self.verify_is_directory()?;
        for entry in std::fs::read_dir(&path)? {
            let entry_path = entry?.path();
            if entry_path.is_dir() {
                std::fs::remove_dir_all(&entry_path)?;
            } else {
                std::fs::remove_file(&entry_path)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // TODO: Check what tests are needed here and what is already covered elsewhere.
    //       The `initialize` method implements the behaviour that is configured
    //       when constructing the `Directory` instance. Many of the corresponding
    //       test cases are currently covered in the constructors tests, but it
    //       might make sense to move them here.

    use super::*;

    use tempfile::tempdir;

    #[test]
    fn initialize() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let path = Path::new(temp_dir.path()).with_subdir("test_dir");
        assert_eq!(path.initialize(), Ok(()));
        let path = path.to_path_buf();

        assert!(path.exists());
        assert!(path.is_dir());
        assert_eq!(path, dir_path);
    }

    #[test]
    fn ensure_exists() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let path = Path::new(&dir_path);
        assert_eq!(path.ensure_exists(), Ok(()));
        let path = path.to_path_buf();

        assert!(path.exists());
        assert!(path.is_dir());
        assert_eq!(path, dir_path);
    }

    #[test]
    fn verify_exists() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let path = Path::new(&dir_path);
        assert!(path.verify_exists().is_err());

        std::fs::create_dir_all(&dir_path).unwrap();
        assert_eq!(path.verify_exists(), Ok(()));
    }

    #[test]
    fn verify_is_directory() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        std::fs::create_dir_all(&dir_path).unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        std::fs::write(&file_path, b"Test content").unwrap();
        let dir = Path::new(&dir_path);
        let non_existing = Path::new(temp_dir.path().join("non_existing_entry"));
        let file = Path::new(&file_path);

        assert_eq!(dir.verify_is_directory(), Ok(()));
        assert!(non_existing.verify_is_directory().is_err());
        assert!(file.verify_is_directory().is_err());
    }

    #[test]
    fn verify_is_file() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        std::fs::create_dir_all(&dir_path).unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        std::fs::write(&file_path, b"Test content").unwrap();
        let dir = Path::new(&dir_path);
        let file = Path::new(&file_path);
        let non_existing = Path::new(temp_dir.path().join("non_existing_entry"));

        assert!(!dir.is_file());
        assert!(file.is_file());
        assert!(!non_existing.is_file());
    }
}
