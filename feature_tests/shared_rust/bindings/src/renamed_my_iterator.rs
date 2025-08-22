pub struct RenamedMyIterator;

impl Drop for RenamedMyIterator {
    fn drop(&mut self) {
        unsafe { RenamedMyIterator_destroy(self) }
    }
}

impl RenamedMyIterator {
    

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RenamedMyIterator_destroy(this : *mut RenamedMyIterator);

}