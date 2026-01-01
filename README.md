# Convenient Working Directories

[![Tests](https://github.com/reihue/conv-wd/actions/workflows/tests.yml/badge.svg?branch=main)](https://github.com/reihue/conv-wd/actions/workflows/tests.yml)

This library provides an abstraction for directories that allows convenient
creation and management of working directories with different lifetimes and behaviors
(e.g. temporary or persistent directories). It also offers some convenience methods
for writing files in various formats (bytes, strings, JSON, TOML).

## Rationale

This library is mainly intended for use during testing and development, especially
as a helper for prototyping when creating data structures.
If e.g. a new data type is developed using [`serde`](https://crates.io/crates/serde),
this library can be used to quickly write out instances of this data type as JSON or
TOML files to see how they are represented in these formats or as test data.

## Features

- Create working directories with different lifetimes:
  - Temporary directories that are deleted when no longer needed.
  - Persistent directories that remain on the filesystem.
  - Directories that are cleaned up on creation.
  - Directories with `.gitignore` files to ignore all content.
- Convenience methods for writing files in various formats:
  - Write raw byte slices or strings.
  - Write JSON files using [`serde_json`](https://crates.io/crates/serde_json).
  - Write TOML files using [`toml`](https://crates.io/crates/toml).
- Integration with Cargo project structure:
  - Create directories relative to the Cargo manifest directory,
    e.g. `target`, `tests`, or `examples` directories.

## Example

```rust
use conv_wd::Directory;

let cargo_manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
let target_dir = cargo_manifest_dir.join("target/demo");

let temp_dir_path = target_dir.join("temp_dir");
let persistent_dir_path = target_dir.join("persistent_dir");
let clean_persistent_dir_path = target_dir.join("clean_persistent_dir");
let persistent_gitignore_dir_path = target_dir.join("persistent_gitignore_dir");

{
    // Create a temporary working directory that is deleted when dropped.
    // This is similar to using `tempfile::tempdir()`, but allows you to specify
    // the path to the directory.
    let temp_dir = Directory::new(&temp_dir_path);
    temp_dir.initialize().unwrap();

    // Create a persistent working directory that remains on the filesystem.
    // This is useful for prototyping data structures and keeping the
    // generated files for later inspection.
    let persistent_dir = Directory::new(&persistent_dir_path).keep();
    persistent_dir.write_string("example.txt", "Hello, persistent world!");
    persistent_dir.initialize().unwrap();

    // Create a persistent working directory that is cleaned up on creation.
    // I.e. any existing content is removed when the `Directory` instance is created.
    // This is useful for ensuring a clean state when reusing directory paths in tests.
    std::fs::create_dir_all(&clean_persistent_dir_path).unwrap();
    std::fs::write(&clean_persistent_dir_path.join("old_file.txt"), "old content").unwrap();
    let clean_temp_dir = Directory::new(&clean_persistent_dir_path).keep().clean();
    clean_temp_dir.initialize().unwrap();

    // Create a persistent working directory with a .gitignore file that ignores all content.
    let persistent_gitignore_dir = Directory::new(&persistent_gitignore_dir_path).keep().with_gitignore();
    persistent_gitignore_dir.initialize().unwrap();
}

assert!(target_dir.exists());
assert!(!temp_dir_path.exists());
assert!(persistent_dir_path.exists());
assert!(persistent_dir_path.join("example.txt").exists());
assert!(clean_persistent_dir_path.exists());
assert!(!clean_persistent_dir_path.join("old_file.txt").exists());
assert!(persistent_gitignore_dir_path.exists());
```
