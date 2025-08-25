pub struct RenamedMyIterable;

impl Drop for RenamedMyIterable {
    fn drop(&mut self) {
        unsafe { namespace_MyIterable_destroy(self) }
    }
}

impl RenamedMyIterable {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_MyIterable_destroy(this : *mut RenamedMyIterable);
}