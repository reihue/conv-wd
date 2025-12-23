/// Asserts that the given path is a relative path.
pub fn assert_relative_path(path: &std::path::Path) {
    if path.is_absolute() {
        panic!(
            "Expected a relative path, but got an absolute path: {}",
            path.display()
        );
    }
}
