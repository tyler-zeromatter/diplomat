pub struct Two;

impl Drop for Two {
    fn drop(&mut self) {
        unsafe { Two_destroy(self) }
    }
}

impl Two {
}

#[link(name = "somelib")]
unsafe extern "C" {
}