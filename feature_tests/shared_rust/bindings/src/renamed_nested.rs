pub struct RenamedNested;

impl Drop for RenamedNested {
    fn drop(&mut self) {
        unsafe { RenamedNested_destroy(self) }
    }
}

impl RenamedNested {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RenamedNested_destroy(this : *mut RenamedNested);
}