pub struct RenamedDeprecatedOpaque;

impl Drop for RenamedDeprecatedOpaque {
    fn drop(&mut self) {
        unsafe { namespace_DeprecatedOpaque_destroy(self) }
    }
}

impl RenamedDeprecatedOpaque {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_DeprecatedOpaque_destroy(this : *mut RenamedDeprecatedOpaque);
}