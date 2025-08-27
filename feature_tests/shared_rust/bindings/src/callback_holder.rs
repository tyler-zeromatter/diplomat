

pub struct CallbackHolder;

impl Drop for CallbackHolder {
    fn drop(&mut self) {
        unsafe { CallbackHolder_destroy(self) }
    }
}

impl CallbackHolder {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn CallbackHolder_destroy(this : *mut CallbackHolder);
}