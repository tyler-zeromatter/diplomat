pub struct RenamedTestOpaque;

impl Drop for RenamedTestOpaque {
    fn drop(&mut self) {
        unsafe { RenamedTestOpaque_destroy(self) }
    }
}

impl RenamedTestOpaque {
}

#[link(name = "somelib")]
unsafe extern "C" {
}