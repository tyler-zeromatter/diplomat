pub struct RenamedOpaqueArithmetic;

impl Drop for RenamedOpaqueArithmetic {
    fn drop(&mut self) {
        unsafe { namespace_OpaqueArithmetic_destroy(self) }
    }
}

impl RenamedOpaqueArithmetic {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_OpaqueArithmetic_destroy(this : *mut RenamedOpaqueArithmetic);
}