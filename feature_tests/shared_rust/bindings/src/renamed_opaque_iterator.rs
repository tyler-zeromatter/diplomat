pub struct RenamedOpaqueIterator;

impl Drop for RenamedOpaqueIterator {
    fn drop(&mut self) {
        unsafe { RenamedOpaqueIterator_destroy(self) }
    }
}

impl RenamedOpaqueIterator {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RenamedOpaqueIterator_destroy(this : *mut RenamedOpaqueIterator);
}