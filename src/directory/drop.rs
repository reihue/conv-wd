use super::*;

impl Drop for Directory {
    fn drop(&mut self) {
        self.path.remove();
    }
}
