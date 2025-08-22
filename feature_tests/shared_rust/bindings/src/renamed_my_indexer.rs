pub struct RenamedMyIndexer;

impl Drop for RenamedMyIndexer {
    fn drop(&mut self) {
        unsafe { RenamedMyIndexer_destroy(self) }
    }
}

impl RenamedMyIndexer {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_MyIndexer_destroy(this : *mut RenamedMyIndexer);
}