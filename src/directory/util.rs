use super::*;

use crate::util::Error;

/// Utility functions.
/// TODO: Revisit naming? Is "util" appropriate here any more?
///       After all, the `initialize` method implements core functionality.
impl Directory {
    /// Creates the directory on the file system if it does not exist.
    /// Also performs cleanup if and creates a `.gitignore` file
    /// if those options are enabled.
    pub fn initialize(&self) -> Result<(), Error> {
        self.ensure_exists()?;
        self.verify_is_directory()?;
        if self.clean_on_init {
            self.remove_contents()?;
        }
        if self.gitignore_on_init {
            self.write_gitignore()?;
        }
        Ok(())
    }

    /// Creates the directory on the file system if it does not exist.
    pub fn ensure_exists(&self) -> Result<(), Error> {
        let path = self.path();
        if !path.exists() {
            std::fs::create_dir_all(&path).map_err(|_| Error::directory_creation_error(path))?;
        }
        Ok(())
    }

    /// Returns an error if `self`'s path does not exist.
    pub fn verify_exists(&self) -> Result<(), Error> {
        let path = self.path();
        if !path.exists() {
            return Err(Error::path_does_not_exist(path));
        }
        Ok(())
    }

    /// Returns an error if `self`'s path is not a directory.
    pub fn verify_is_directory(&self) -> Result<(), Error> {
        let path = self.path();
        if !path.is_dir() {
            return Err(Error::path_is_not_a_directory(path));
        }
        Ok(())
    }

    /// Removes all content of the directory.
    /// Returns an error if the directory does not exist or is not a directory.
    pub fn remove_contents(&self) -> Result<(), Error> {
        let path = self.path();
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

        let directory = Directory::new(temp_dir.path()).new_subdir("test_dir");
        assert_eq!(directory.initialize(), Ok(()));
        let path = directory.path();

        assert!(path.exists());
        assert!(path.is_dir());
        assert_eq!(path, dir_path);
    }
}
