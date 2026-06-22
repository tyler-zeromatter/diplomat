pub struct RenamedOpaqueZSTIndexer;

impl Drop for RenamedOpaqueZSTIndexer {
    fn drop(&mut self) {
        unsafe { namespace_OpaqueZSTIndexer_destroy(self) }
    }
}

impl RenamedOpaqueZSTIndexer {
    pub fn new() -> Box<RenamedOpaqueZSTIndexer> {
        let ret = unsafe { namespace_OpaqueZSTIndexer_new() };
        
        ret

    }

    pub fn index<'anon_0>(&'anon_0 self, idx : usize) -> Option<Box<RenamedOpaqueZSTIndexer>> {
        let ret = unsafe { namespace_OpaqueZSTIndexer_index(self, idx) };
        
        ret

    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_OpaqueZSTIndexer_new() -> Box<RenamedOpaqueZSTIndexer>;

    fn namespace_OpaqueZSTIndexer_index<'anon_0>(this: &'anon_0 RenamedOpaqueZSTIndexer, idx : usize) -> Option<Box<RenamedOpaqueZSTIndexer>>;

    fn namespace_OpaqueZSTIndexer_destroy(this : *mut RenamedOpaqueZSTIndexer);
}