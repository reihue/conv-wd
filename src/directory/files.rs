use super::*;

use serde::Serialize;
use std::path::Path;

use crate::util::Error;

/// Methods for file operations within the directory.
impl Directory {
    /// Writes a byte slice to a file at the given path within the directory.
    pub fn write_bytes<P: AsRef<Path>, C: AsRef<[u8]>>(
        &self,
        relative_path: P,
        content: C,
    ) -> Result<()> {
        if relative_path.as_ref().is_absolute() {
            return Err(Error::path_is_absolute(relative_path));
        }
        let file_path = self.path().join(relative_path);
        std::fs::write(&file_path, content.as_ref())?;

        Ok(())
    }

    /// Writes a string to a file at the given path within the directory.
    pub fn write_string<P: AsRef<Path>, S: Into<String>>(
        &self,
        relative_path: P,
        content: S,
    ) -> Result<()> {
        self.write_bytes(relative_path, content.into().as_bytes())
    }

    /// Writes a serde-serializable object as JSON to a file at the given path within the directory.
    /// Adds the `.json` extension to the file name if not already present (overwrites existing extension).
    pub fn write_json<P: AsRef<Path>, T: Serialize>(
        &self,
        relative_path: P,
        obj: &T,
    ) -> Result<()> {
        let json_string = serde_json::to_string_pretty(obj)?;
        self.write_string(relative_path.as_ref().with_extension("json"), json_string)
    }

    /// Writes a serde-serializable object as TOML to a file at the given path within the directory.
    /// Adds the `.toml` extension to the file name if not already present (replaces existing extension).
    pub fn write_toml<P: AsRef<Path>, T: Serialize>(
        &self,
        relative_path: P,
        obj: &T,
    ) -> Result<()> {
        let toml_string = toml::to_string_pretty(obj)?;
        self.write_string(relative_path.as_ref().with_extension("toml"), toml_string)
    }

    /// Convenience method to write a `.gitignore` file in the directory
    /// that causes all content to be ignored by Git.
    pub fn write_gitignore(&self) -> Result<()> {
        self.write_string(".gitignore", "*\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn write_bytes() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let directory = Directory::new(dir_path.join("subdir"));
        assert_eq!(directory.initialize(), Ok(()));

        let file_name = "test_file.txt";
        let file_content = b"Hello, world!";
        assert_eq!(directory.write_bytes(file_name, file_content), Ok(()));

        let written_file_path = directory.path().join(file_name);
        assert!(written_file_path.exists());
        let read_content = std::fs::read(&written_file_path).unwrap();
        assert_eq!(read_content, file_content);
    }

    #[test]
    fn write_string() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let directory = Directory::new(dir_path.join("subdir"));
        assert_eq!(directory.initialize(), Ok(()));

        let file_name = "test_file.txt";
        let file_content = "Hello, world!";
        assert_eq!(directory.write_string(file_name, file_content), Ok(()));
        let written_file_path = directory.path().join(file_name);
        assert!(written_file_path.exists());
        let read_content = std::fs::read_to_string(&written_file_path).unwrap();
        assert_eq!(read_content, file_content);
    }

    #[test]
    fn write_gitignore() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        let directory = Directory::new(dir_path.join("subdir"));
        assert_eq!(directory.initialize(), Ok(()));

        assert_eq!(directory.write_gitignore(), Ok(()));
        let written_file_path = directory.path().join(".gitignore");
        assert!(written_file_path.exists());
        let read_content = std::fs::read_to_string(&written_file_path).unwrap();
        assert_eq!(read_content, "*\n");
    }

    #[derive(Serialize, serde::Deserialize, PartialEq, Debug)]
    struct TestData {
        content: String,
    }

    #[test]
    fn write_json() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        let directory = Directory::new(dir_path.join("subdir"));
        assert_eq!(directory.initialize(), Ok(()));
        let testdata = TestData {
            content: "Hello, JSON!".to_string(),
        };
        assert_eq!(directory.write_json("data_file1", &testdata), Ok(()));
        assert_eq!(directory.write_json("data_file2.json", &testdata), Ok(()));
        assert_eq!(directory.write_json("data_file3.txt", &testdata), Ok(()));

        assert!(directory.path().join("data_file1.json").exists());
        assert!(directory.path().join("data_file2.json").exists());
        assert!(directory.path().join("data_file3.json").exists());

        for file_name in &["data_file1.json", "data_file2.json", "data_file3.json"] {
            let written_file_path = directory.path().join(file_name);
            let read_content = std::fs::read_to_string(&written_file_path).unwrap();
            let deserialized: TestData =
                serde_json::from_str(&read_content).expect("Failed to deserialize JSON");
            assert_eq!(deserialized, testdata);
        }
    }

    #[test]
    fn write_toml() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        let directory = Directory::new(dir_path.join("subdir"));
        assert_eq!(directory.initialize(), Ok(()));

        let testdata = TestData {
            content: "Hello, TOML!".to_string(),
        };
        assert_eq!(directory.write_toml("data_file1", &testdata), Ok(()));
        assert_eq!(directory.write_toml("data_file2.toml", &testdata), Ok(()));
        assert_eq!(directory.write_toml("data_file3.txt", &testdata), Ok(()));

        assert!(directory.path().join("data_file1.toml").exists());
        assert!(directory.path().join("data_file2.toml").exists());
        assert!(directory.path().join("data_file3.toml").exists());

        for file_name in &["data_file1.toml", "data_file2.toml", "data_file3.toml"] {
            let written_file_path = directory.path().join(file_name);
            let read_content = std::fs::read_to_string(&written_file_path).unwrap();
            let deserialized: TestData =
                toml::from_str(&read_content).expect("Failed to deserialize JSON");
            assert_eq!(deserialized, testdata);
        }
    }
}
