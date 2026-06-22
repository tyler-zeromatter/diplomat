pub struct RenamedOpaqueRefIterable;

impl Drop for RenamedOpaqueRefIterable {
    fn drop(&mut self) {
        unsafe { namespace_OpaqueRefIterable_destroy(self) }
    }
}

impl RenamedOpaqueRefIterable {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_OpaqueRefIterable_destroy(this : *mut RenamedOpaqueRefIterable);
}