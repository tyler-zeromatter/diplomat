pub struct RenamedOpaqueZST;

impl Drop for RenamedOpaqueZST {
    fn drop(&mut self) {
        unsafe { namespace_OpaqueZST_destroy(self) }
    }
}

impl RenamedOpaqueZST {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_OpaqueZST_destroy(this : *mut RenamedOpaqueZST);
}