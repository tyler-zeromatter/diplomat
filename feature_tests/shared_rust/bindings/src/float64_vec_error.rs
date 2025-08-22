pub struct Float64VecError;

impl Drop for Float64VecError {
    fn drop(&mut self) {
        unsafe { Float64VecError_destroy(self) }
    }
}

impl Float64VecError {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Float64VecError_destroy(this : *mut Float64VecError);
}