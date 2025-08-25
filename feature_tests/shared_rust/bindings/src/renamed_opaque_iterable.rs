pub struct RenamedOpaqueIterable;

impl Drop for RenamedOpaqueIterable {
    fn drop(&mut self) {
        unsafe { namespace_OpaqueIterable_destroy(self) }
    }
}

impl RenamedOpaqueIterable {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_OpaqueIterable_destroy(this : *mut RenamedOpaqueIterable);
}