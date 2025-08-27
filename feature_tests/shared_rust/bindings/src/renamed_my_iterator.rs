pub struct RenamedMyIterator<'a>;

impl Drop for RenamedMyIterator {
    fn drop(&mut self) {
        unsafe { namespace_MyIterator_destroy(self) }
    }
}

impl<'a> RenamedMyIterator<'a> {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_MyIterator_destroy(this : *mut RenamedMyIterator);
}