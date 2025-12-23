# Convenient Working Directories

This library provides an abstraction for directories that is mainly intended
for use during testing and development. It allows to create working directories
with different lifetimes and behaviors, such as temporary directories,
persistent directories, and directories that are cleaned up on creation.
These directory abstractions also provide some convenience methods for common
file operations and there are constructors for creating working directories
inside a cargo project's repository.
