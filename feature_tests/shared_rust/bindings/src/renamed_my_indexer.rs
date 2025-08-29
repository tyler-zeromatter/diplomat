pub struct RenamedMyIndexer;

impl Drop for RenamedMyIndexer {
    fn drop(&mut self) {
        unsafe { namespace_MyIndexer_destroy(self) }
    }
}

impl RenamedMyIndexer {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_MyIndexer_destroy(this : *mut RenamedMyIndexer);
}