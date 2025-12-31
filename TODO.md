# To Do

- Demonstrate usage
  - with `?` operator in examples and tests (e.g. for quick prototyping).
  - logging errors instead of panicking in production code
- Add convenience constructors without the extra path, e.g. for:
  - an absolute path
  - the current working directory
  - the user's home directory
  - temporary directories (system or user)
  - cargo subdirs `examples`, `tests`, `target`, etc.
- Add safety checks for directory operations:
  - Ensure that the path is a directory (not a file or symlink) when creating a `Directory` instance.
  - Ensure that the directory is empty before removing it on drop.
  - Avoid removing important directories (e.g. root directory, home directory, etc.).
  - Provide options to override these safety checks if needed.
    - Use feature flags?
    - Use a factory pattern for creating `Directory` instances with different safety levels?
    - Check with behaviour of similar libraries/crates or the standard library.
- At any time, handle the case that the directory path has become invalid
  (e.g. does not exist any more or is no longer a directory).
  - Return results from access methods?
  - Is `ensure_exists` allowed to automatically recreate the directory?
  - Do not fail on drop (log errors?).
  - Provide a method to check if the directory still exists.
  - Provide a method to recreate the directory if it has been removed?
- Tests: Add more descriptive tests that better show what happens in each case.
  - Use properly named helper functions/macros for setting up and verifying
    test conditions in unit tests.
  - Use the library itself in integration tests to demonstrate its usage
    for creating test data and temporary directories.
    Verify that cleanup policies work as expected.
- Tests: Add integration tests that check all use cases, e.g.:
  - Git-ignored directories for test data that are persistent for inspection after test runs,
    but that are cleaned up before test runs.
  - Git-ignored output directories for data examples.
  - Comparison of test data with generated example data.
  - Also check cleanup policy in case of multiple temporary directories
    that share a common ancestor.
  - See also `examples/` directory.
