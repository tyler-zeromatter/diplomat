pub struct Two<'a, 'b>;

impl<'a, 'b> Drop for Two<'a, 'b> {
    fn drop(&mut self) {
        unsafe { Two_destroy(self) }
    }
}

impl<'a, 'b> Two<'a, 'b> {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Two_destroy(this : *mut Two);
}