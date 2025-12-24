# To Do

## Constructors

- Explicit constructors for cargo subdirs `examples`, `tests`, `target`, etc.
- Store a base path along with an optional relative subpath.
  - If temporary, remove all subdirectories on drop, if they exist and are empty.

## Error Handling

- Use `Result` return types instead of panicking in case of errors.
- At any time, handle the case that the complete path (base + subpath) has been
  invalidated (i.e. does not exist any more):
  - Log errors on access methods.
  - Is `ensure_exists` allowed to automatically recreate the directory?
  - Do not fail on drop, but also log errors.
  - Provide a method to check if the directory still exists.
  - Provide a method to recreate the directory if it has been removed?

## Testing

- Make tests more descriptive by providing meaningful helper functions/macros
  when setting up and verifying test conditions.
- Add integration tests that check all use cases, e.g.:
  - Git-ignored directories for test data that are persistent for inspection after test runs,
    but that are cleaned up before test runs.
  - Git-ignored output directories for data examples.
  - Comparison of test data with generated example data.
  - Also check cleanup policy in case of multiple siblings temporary directories
    that share a common ancestor.
  - See also `examples/` directory.
