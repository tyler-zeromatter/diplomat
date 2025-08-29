pub struct RenamedTestOpaque;

impl Drop for RenamedTestOpaque {
    fn drop(&mut self) {
        unsafe { namespace_TestOpaque_destroy(self) }
    }
}

impl RenamedTestOpaque {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_TestOpaque_destroy(this : *mut RenamedTestOpaque);
}