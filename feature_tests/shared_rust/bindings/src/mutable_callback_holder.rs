pub struct MutableCallbackHolder;

impl Drop for MutableCallbackHolder {
    fn drop(&mut self) {
        unsafe { MutableCallbackHolder_destroy(self) }
    }
}

impl MutableCallbackHolder {
}

#[link(name = "somelib")]
unsafe extern "C" {
}