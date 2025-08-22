pub struct CallbackHolder;

impl Drop for CallbackHolder {
    fn drop(&mut self) {
        unsafe { CallbackHolder_destroy(self) }
    }
}

impl CallbackHolder {
}

#[link(name = "somelib")]
unsafe extern "C" {
}