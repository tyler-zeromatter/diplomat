pub struct RenamedNested;

impl Drop for RenamedNested {
    fn drop(&mut self) {
        unsafe { namespace_Nested_destroy(self) }
    }
}

impl RenamedNested {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_Nested_destroy(this : *mut RenamedNested);
}