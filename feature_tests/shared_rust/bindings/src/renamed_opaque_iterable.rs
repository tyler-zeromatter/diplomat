pub struct RenamedOpaqueIterable;

impl Drop for RenamedOpaqueIterable {
    fn drop(&mut self) {
        unsafe { RenamedOpaqueIterable_destroy(self) }
    }
}

impl RenamedOpaqueIterable {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RenamedOpaqueIterable_destroy(this : *mut RenamedOpaqueIterable);
}