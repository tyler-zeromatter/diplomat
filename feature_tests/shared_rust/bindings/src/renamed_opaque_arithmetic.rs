pub struct RenamedOpaqueArithmetic;

impl Drop for RenamedOpaqueArithmetic {
    fn drop(&mut self) {
        unsafe { RenamedOpaqueArithmetic_destroy(self) }
    }
}

impl RenamedOpaqueArithmetic {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RenamedOpaqueArithmetic_destroy(this : *mut RenamedOpaqueArithmetic);
}