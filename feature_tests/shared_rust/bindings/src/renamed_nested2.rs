pub struct RenamedNested2;

impl Drop for RenamedNested2 {
    fn drop(&mut self) {
        unsafe { RenamedNested2_destroy(self) }
    }
}

impl RenamedNested2 {
}

#[link(name = "somelib")]
unsafe extern "C" {
}