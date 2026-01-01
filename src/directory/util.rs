use super::*;

/// Utility functions.
/// TODO: Revisit naming? Is "util" still appropriate here?
///       After all, the `initialize` method implements core functionality.
impl Directory {
    /// Creates the directory on the file system if it does not exist.
    /// Also performs cleanup if and creates a `.gitignore` file
    /// if those options are enabled.
    pub fn initialize(&self) -> Result<()> {
        self.path.initialize()?;
        if self.clean_on_init {
            self.remove_contents()?;
        }
        if self.gitignore_on_init {
            self.write_gitignore()?;
        }
        Ok(())
    }

    /// Removes all content of the directory.
    /// Returns an error if the directory does not exist or is not a directory.
    pub fn remove_contents(&self) -> Result<()> {
        let path = self.path();
        self.path.verify_exists()?;
        self.path.verify_is_directory()?;
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
