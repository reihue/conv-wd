use super::*;

use serde::Serialize;
use std::path::Path;

/// Methods for file operations within the directory.
impl Directory {
    /// Writes a byte slice to a file at the given path within the directory.
    /// Panics if the path is absolute or if the write operation fails.
    pub fn write_bytes<P: AsRef<Path>, C: AsRef<[u8]>>(&self, relative_path: P, content: C) {
        assert!(!relative_path.as_ref().is_absolute());
        let file_path = self.path().join(relative_path.as_ref());
        std::fs::write(&file_path, content.as_ref())
            .unwrap_or_else(|e| panic!("Failed to write to file at {}: {e}", file_path.display()));
    }

    /// Writes a string to a file at the given path within the directory.
    /// Panics if the path is absolute or if the write operation fails.
    pub fn write_string<P: AsRef<Path>, S: Into<String>>(&self, relative_path: P, content: S) {
        self.write_bytes(relative_path, content.into().as_bytes());
    }

    /// Writes a serde-serializable object as JSON to a file at the given path within the directory.
    /// Adds the `.json` extension to the file name if not already present (overwrites existing extension).
    /// Panics if the path is absolute or if the serialization or write operation fails.
    pub fn write_json<P: AsRef<Path>, T: Serialize>(&self, relative_path: P, obj: &T) {
        self.write_string(
            relative_path.as_ref().with_extension("json"),
            serde_json::to_string_pretty(obj).unwrap_or_else(|e| {
                panic!(
                    "Failed to serialize object to JSON for file at {}: {e}",
                    relative_path.as_ref().display()
                )
            }),
        );
    }

    /// Writes a serde-serializable object as TOML to a file at the given path within the directory.
    /// Adds the `.toml` extension to the file name if not already present (replaces existing extension).
    /// Panics if the path is absolute or if the serialization or write operation fails.
    pub fn write_toml<P: AsRef<Path>, T: Serialize>(&self, relative_path: P, obj: &T) {
        self.write_string(
            relative_path.as_ref().with_extension("toml"),
            toml::to_string_pretty(obj).unwrap_or_else(|e| {
                panic!(
                    "Failed to serialize object to TOML for file at {}: {e}",
                    relative_path.as_ref().display()
                )
            }),
        );
    }

    /// Convenience method to write a `.gitignore` file in the directory
    /// that causes all content to be ignored by Git.
    /// Panics if the write operation fails.
    pub fn write_gitignore(&self) {
        self.write_string(".gitignore", "*\n");
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
        let file_name = "test_file.txt";
        let file_content = b"Hello, world!";
        directory.write_bytes(file_name, file_content);

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
        let file_name = "test_file.txt";
        let file_content = "Hello, world!";
        directory.write_string(file_name, file_content);

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
        directory.write_gitignore();

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

        let testdata = TestData {
            content: "Hello, JSON!".to_string(),
        };
        directory.write_json("data_file1", &testdata);
        directory.write_json("data_file2.json", &testdata);
        directory.write_json("data_file3.txt", &testdata);

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

        let testdata = TestData {
            content: "Hello, TOML!".to_string(),
        };
        directory.write_toml("data_file1", &testdata);
        directory.write_toml("data_file2.toml", &testdata);
        directory.write_toml("data_file3.txt", &testdata);

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
