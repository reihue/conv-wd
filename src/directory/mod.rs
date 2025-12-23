use std::path::PathBuf;

/// Represents a directory in the file system.
/// The actual directory is created on the file system when this struct is instantiated.
/// By default, the directory is persistent, but there are options to make it temporary.
pub struct Directory {
    path: PathBuf,
    keep_on_drop: bool,
}

mod access;
mod cargo;
mod constructors;
mod drop;
mod files;
mod util;
