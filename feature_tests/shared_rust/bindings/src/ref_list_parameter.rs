pub struct RefListParameter;

impl Drop for RefListParameter {
    fn drop(&mut self) {
        unsafe { RefListParameter_destroy(self) }
    }
}

impl RefListParameter {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RefListParameter_destroy(this : *mut RefListParameter);
}