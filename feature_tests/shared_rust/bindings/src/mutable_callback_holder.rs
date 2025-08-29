pub struct MutableCallbackHolder;

impl Drop for MutableCallbackHolder {
    fn drop(&mut self) {
        unsafe { MutableCallbackHolder_destroy(self) }
    }
}

impl MutableCallbackHolder {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn MutableCallbackHolder_destroy(this : *mut MutableCallbackHolder);
}