pub struct Two;

impl Drop for Two {
    fn drop(&mut self) {
        unsafe { Two_destroy(self) }
    }
}

impl Two {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Two_destroy(this : *mut Two);
}