use super::*;

use std::path::Path;

/// Convenience methods/constructors for working with Cargo projects.
impl Directory {
    /// Creates a new `Directory` instance representing a
    /// subdirectory of the cargo manifest directory.
    /// The directory is created if it does not exist.
    /// the subdirectory path is an absolute path, invalid,
    /// or if the directory cannot be created.
    ///
    /// # Arguments
    /// * `subdir` - The subdirectory path relative to the cargo manifest directory.
    ///
    /// # Example
    /// ```rust
    /// use conv_wd::Directory;
    /// use std::path::Path;
    ///
    /// let cargo_subdir = Directory::cargo_manifest_subdir("target/my_cargo_subdir");
    ///
    /// assert_eq!(
    ///   cargo_subdir.path(),
    ///   Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("target/my_cargo_subdir")
    /// );
    /// ```
    pub fn cargo_manifest_subdir<P: AsRef<Path>>(subdir: P) -> Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable is not set");
        assert!(!subdir.as_ref().is_absolute());
        let path = std::path::Path::new(&manifest_dir).join(subdir.as_ref());
        Directory::new(path)
    }

    /// Creates a new `Directory` instance under the `examples`
    /// subdirectory of the cargo manifest directory.
    /// The directory is created if it does not exist.
    /// or if the directory cannot be created.
    ///
    /// # Example
    /// ```rust
    /// use conv_wd::Directory;
    /// use std::path::Path;
    ///
    /// let examples_dir = Directory::cargo_examples_subdir("my_subdir");
    ///
    /// assert_eq!(
    ///   examples_dir.path(),
    ///   Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("examples/my_subdir")
    /// );
    /// ```
    pub fn cargo_examples_subdir<P: AsRef<Path>>(subdir: P) -> Self {
        Self::cargo_manifest_subdir(PathBuf::from("examples").join(subdir.as_ref()))
    }

    /// Creates a persistent `Directory` instance under the `tests`
    /// subdirectory of the cargo manifest directory.
    /// The directory is created if it does not exist.
    /// or if the directory cannot be created.
    ///
    /// # Example
    /// ```rust
    /// use conv_wd::Directory;
    /// use std::path::Path;
    ///
    /// let tests_dir = Directory::cargo_tests_subdir("my_subdir");
    ///
    /// assert_eq!(
    ///   tests_dir.path(),
    ///   Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests/my_subdir")
    /// );
    /// ```
    pub fn cargo_tests_subdir<P: AsRef<Path>>(subdir: P) -> Self {
        Self::cargo_manifest_subdir(PathBuf::from("tests").join(subdir.as_ref()))
    }

    /// Creates a persistent `Directory` instance under the `target`
    /// subdirectory of the cargo manifest directory.
    /// The directory is created if it does not exist.
    /// or if the directory cannot be created.
    ///
    /// # Example
    /// ```rust
    /// use conv_wd::Directory;
    /// use std::path::Path;
    ///
    /// let target_dir = Directory::cargo_target_subdir("my_subdir");
    ///
    /// assert_eq!(
    ///   target_dir.path(),
    ///   Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("target/my_subdir")
    /// );
    /// ```
    pub fn cargo_target_subdir<P: AsRef<Path>>(subdir: P) -> Self {
        Self::cargo_manifest_subdir(PathBuf::from("target").join(subdir.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_manifest_subdir() {
        let subdir_name = "target/cargo_manifest_testdirs";
        let expected_path = std::path::Path::new(
            &std::env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_DIR environment variable is not set"),
        )
        .join(subdir_name);

        {
            let directory = Directory::cargo_manifest_subdir(subdir_name);
            assert!(!expected_path.exists());
            assert_eq!(directory.initialize(), Ok(()));

            assert_eq!(directory.path(), expected_path.as_path());
            assert!(expected_path.exists());
            assert!(expected_path.is_dir());
        }
        assert!(!expected_path.exists());
    }
}
