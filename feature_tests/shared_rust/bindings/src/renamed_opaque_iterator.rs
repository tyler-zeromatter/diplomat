pub struct RenamedOpaqueIterator;

impl Drop for RenamedOpaqueIterator {
    fn drop(&mut self) {
        unsafe { namespace_OpaqueIterator_destroy(self) }
    }
}

impl RenamedOpaqueIterator {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_OpaqueIterator_destroy(this : *mut RenamedOpaqueIterator);
}