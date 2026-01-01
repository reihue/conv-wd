use std::path::PathBuf;

use crate::util::{Path, Result};

const CLEAN_ON_INIT_DEFAULT: bool = false;
const GITIGNORE_ON_INIT_DEFAULT: bool = false;

/// Represents a directory in the file system.
///
/// This struct provides functionality to create and clean up directories,
/// and read/write files within them.
///
/// On construction, determines wich parts of the path already exist and which
/// parts need to be created. Internally stores this information to be able to
/// initialize the directory structore when needed, and to clean up created
/// directories on drop.
///
/// Factory methods can be used for more fine grained control over the creation
/// and cleanup behavior. E.g. to create persistent directories that are not
/// removed on drop, to clean them up on creation, or to make them ignored by Git.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directory {
    /// The path used for this directory.
    path: Path,
    /// If true, any existing content will be removed when the directory is initialized.
    pub clean_on_init: bool,
    /// If true, a `.gitignore` file will be created in the directory on initialization.
    pub gitignore_on_init: bool,
}

mod cargo;
mod constructors;
mod drop;
mod files;
mod properties;
mod util;

// TODO: add more tests
// - new_subdir
// - new_persistent
// - more complex paths
// - more complex drop behaviour (e.g. non-empty created directories,
//.  multiple `Directory` instances with common ancestors)
