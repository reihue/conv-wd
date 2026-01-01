use conv_wd::Directory;

const BASE_DIRNAME: &str = "usecase_testdirs";

/// Gets a `std::path::PathBuf` for the base directory under `tests/<BASE_DIRNAME>`.
/// Note: All tests in this file create/use directories below this base directory.
/// It is expected that the base directory already exists.
fn base_dir_path() -> std::path::PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    std::path::Path::new(&manifest_dir)
        .join("tests")
        .join(BASE_DIRNAME)
}

/// Test Case: Volatile Empty Directory
/// - Create a `Directory` instance for a path that does not exist, yet.
/// - The directory will be created on initialization and removed on drop.
/// - No filesystem operations are performed on the directory.
#[test]
fn volatile_empty_directory() {
    let path = base_dir_path().join("volatile_empty_dir");
    {
        // Create the `Directory` instance.
        let dir = Directory::new(&path);

        // Verify that the directory does not exist yet.
        assert!(!dir.path().exists());

        // Initialize the directory (create if not existing).
        assert_eq!(dir.initialize(), Ok(()));

        // Verify that the directory exists and is a directory.
        // Use methods from both `Directory` and `std::path::Path`
        // for verification.
        let path = dir.path();
        assert!(path.exists());
        assert!(path.is_dir());
        assert!(dir.exists());
        assert!(dir.is_dir());
    }

    // Verify that the directory has been removed when the above scope ended.
    assert!(!path.exists());
}

/// Test Case: Persistent Empty Directory
/// - Create a `Directory` instance for a path that does not exist, yet.
/// - The directory will be created on initialization and kept on drop.
/// - No filesystem operations are performed on the directory.
#[test]
fn persistent_empty_directory() {
    let path = base_dir_path().join("persistent_empty_dir");
    {
        // Create the `Directory` instance.
        let dir = Directory::new(&path).keep();

        // Initialize the directory (create if not existing).
        assert_eq!(dir.initialize(), Ok(()));

        // Verify that the directory exists and is a directory.
        // Use various methods from both `Directory`
        // and `std::path::Path` for verification.
        assert!(path.exists());
        assert!(path.is_dir());
        assert!(dir.path().exists());
        assert!(dir.path().is_dir());
        assert!(dir.exists());
        assert!(dir.is_dir());
    }

    // Verify that the directory still exists after the above scope ended.
    assert!(path.exists());
    assert!(path.is_dir());
}

/// Test Case: Volatile Non-Empty Directory
/// - Create a `Directory` instance for a path that does not exist, yet.
/// - The directory will be created on initialization and removed on drop.
/// - Create some files and subdirectories inside the directory.
/// - Verify that the files and subdirectories are also removed on drop.
#[test]
fn volatile_non_empty_directory() {
    let path = base_dir_path().join("volatile_non_empty_dir");
    {
        // Create the `Directory` instance.
        let dir = Directory::new(&path);

        // Verify that the directory does not exist yet.
        assert!(!dir.path().exists());

        // Initialize the directory (create if not existing).
        assert_eq!(dir.initialize(), Ok(()));

        // Create some files and subdirectories inside the directory.
        dir.write_string("test_file.txt", "Test content").unwrap();
        std::fs::create_dir_all(dir.path().join("subdir")).unwrap();
        dir.write_string("subdir/subfile.txt", "Subdirectory file content")
            .unwrap();

        // The following paths should now exist:
        let file_path = dir.path().join("test_file.txt");
        let subdir_path = dir.path().join("subdir");
        let subfile_path = subdir_path.join("subfile.txt");

        // Verify that the files and subdirectories exist.
        // Use various methods from both `Directory`
        // and `std::path::Path` for verification.
        assert!(path.exists());
        assert!(path.is_dir());
        assert!(file_path.exists());
        assert!(file_path.is_file());
        assert!(subdir_path.exists());
        assert!(subdir_path.is_dir());
        assert!(subfile_path.exists());
        assert!(subfile_path.is_file());
        assert!(dir.exists());
        assert!(dir.is_dir());
    }

    // Verify that the directory and its contents have been removed when the above scope ended.
    assert!(!path.exists());
}

/// Test Case: Persistent Non-Empty Directory
/// - Create a `Directory` instance for a path that does not exist, yet.
/// - The directory will be created on initialization and kept on drop.
/// - Create some files and subdirectories inside the directory.
/// - Verify that the files and subdirectories are still present after drop.
#[test]
fn persistent_non_empty_directory() {
    let path = base_dir_path().join("persistent_non_empty_dir");
    let timestamp = chrono::Utc::now().to_rfc3339();
    {
        // Create the `Directory` instance.
        let dir = Directory::new(&path).keep();

        // Initialize the directory (create if not existing).
        assert_eq!(dir.initialize(), Ok(()));

        // Create some files inside the directory.
        // - A .gitignore file that ensures that all contents are ignored by git.
        // - A text file with some content. The file content includes
        //   the current timestamp. This allows verifying that the file
        //   was created during the test run and not before.
        dir.write_string(".gitignore", "*").unwrap();
        dir.write_string(
            "test_file.txt",
            &format!("Test content created at {}", timestamp),
        )
        .unwrap();
    }

    // Verify that the directory and its contents still exist after the above scope ended.
    assert!(path.exists());
    assert!(path.is_dir());
    let file_path = path.join("test_file.txt");
    assert!(file_path.exists());
    assert!(file_path.is_file());
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains(&timestamp));
}
