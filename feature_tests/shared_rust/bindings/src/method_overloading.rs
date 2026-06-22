pub struct MethodOverloading;

impl Drop for MethodOverloading {
    fn drop(&mut self) {
        unsafe { MethodOverloading_destroy(self) }
    }
}

impl MethodOverloading {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn MethodOverloading_destroy(this : *mut MethodOverloading);
}