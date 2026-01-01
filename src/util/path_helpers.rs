use std::path::{Path, PathBuf};

use crate::util::{Error, Result};

/// Returns a tuple consisting of:
/// - The path to the closest existing ancestor of the given path.
///   (Empty if no such ancestor exists.)
/// - A vector of subdirectory names from that ancestor to the original path.
pub fn closest_ancestor<P: AsRef<Path>>(path: P) -> Result<(PathBuf, Vec<String>)> {
    // Recursive function to find the closest existing ancestor.
    if path.as_ref().exists() {
        return Ok((path.as_ref().to_path_buf(), Vec::new()));
    }

    let parent = match path.as_ref().parent() {
        Some(p) => p,
        None => return Ok((PathBuf::new(), Vec::new())),
    };

    let name = match path.as_ref().file_name() {
        Some(n) => n.to_string_lossy().to_string(),
        None => return Err(Error::malformed_path(path)),
    };

    let (ancestor, mut subdirs) = closest_ancestor(parent)?;
    subdirs.push(name);
    Ok((ancestor, subdirs))
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn closest_ancestor_existing_dir() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("existing_dir");
        std::fs::create_dir_all(&dir_path).unwrap();

        let ancestor = closest_ancestor(&dir_path);
        assert_eq!(ancestor, Ok((dir_path.clone(), Vec::new())));
    }

    #[test]
    fn closest_ancestor_parent_exists() {
        let temp_dir = tempdir().unwrap();
        let parent_path = temp_dir.path().join("parent_dir");
        std::fs::create_dir_all(&parent_path).unwrap();
        let child_path = parent_path.join("child_dir");

        let ancestor = closest_ancestor(&child_path);
        assert_eq!(
            ancestor,
            Ok((parent_path.clone(), vec!["child_dir".to_string()]))
        );
    }

    #[test]
    fn closest_ancestor_grandparent_exists() {
        let temp_dir = tempdir().unwrap();
        let grandparent_path = temp_dir.path().join("grandparent_dir");
        std::fs::create_dir_all(&grandparent_path).unwrap();
        let parent_path = grandparent_path.join("parent_dir");
        let child_path = parent_path.join("child_dir");

        let ancestor = closest_ancestor(&child_path);
        assert_eq!(
            ancestor,
            Ok((
                grandparent_path.clone(),
                vec!["parent_dir".to_string(), "child_dir".to_string()]
            ))
        );
    }

    #[test]
    fn closest_ancestor_root() {
        let root_path = PathBuf::from("/");

        let ancestor = closest_ancestor(&root_path);
        assert_eq!(ancestor, Ok((root_path.clone(), Vec::new())));
    }

    #[test]
    fn closest_ancestor_empty() {
        let empty_path = PathBuf::from("");

        let ancestor = closest_ancestor(&empty_path);
        assert_eq!(ancestor, Ok((PathBuf::new(), Vec::new())));
    }

    #[test]
    fn closest_ancestor_single_relative() {
        let single_relative_path = PathBuf::from("non_existing_dir");

        let ancestor = closest_ancestor(&single_relative_path);
        assert_eq!(
            ancestor,
            Ok((PathBuf::new(), vec!["non_existing_dir".to_string()]))
        );
    }

    #[test]
    fn closest_ancestor_multiple_relative() {
        let multiple_relative_path = PathBuf::from("dir1/dir2/dir3");

        let ancestor = closest_ancestor(&multiple_relative_path);
        assert_eq!(
            ancestor,
            Ok((
                PathBuf::new(),
                vec!["dir1".to_string(), "dir2".to_string(), "dir3".to_string()]
            ))
        );
    }

    #[test]
    fn closest_ancestor_no_filename() {
        let path_with_no_filename = PathBuf::from("some/path/..");

        let ancestor = closest_ancestor(&path_with_no_filename);
        assert_eq!(ancestor, Err(Error::malformed_path(path_with_no_filename)));
    }
}
