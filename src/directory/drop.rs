use super::*;

impl Drop for Directory {
    fn drop(&mut self) {
        if self.is_persistent() {
            return;
        }
        let _ = self.remove_contents();
        self.path.remove();
    }
}
