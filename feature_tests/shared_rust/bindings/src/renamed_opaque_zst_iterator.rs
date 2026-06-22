pub struct RenamedOpaqueZSTIterator;

impl Drop for RenamedOpaqueZSTIterator {
    fn drop(&mut self) {
        unsafe { namespace_OpaqueZSTIterator_destroy(self) }
    }
}

impl RenamedOpaqueZSTIterator {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_OpaqueZSTIterator_destroy(this : *mut RenamedOpaqueZSTIterator);
}