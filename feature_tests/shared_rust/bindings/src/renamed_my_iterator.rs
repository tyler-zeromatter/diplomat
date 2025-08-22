pub struct RenamedMyIterator;

impl Drop for RenamedMyIterator {
    fn drop(&mut self) {
        unsafe { namespace_MyIterator_destroy(self) }
    }
}

impl RenamedMyIterator {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_MyIterator_destroy(this : *mut RenamedMyIterator);
}