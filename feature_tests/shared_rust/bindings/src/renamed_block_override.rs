pub struct RenamedBlockOverride;

impl Drop for RenamedBlockOverride {
    fn drop(&mut self) {
        unsafe { namespace_BlockOverride_destroy(self) }
    }
}

impl RenamedBlockOverride {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_BlockOverride_destroy(this : *mut RenamedBlockOverride);
}