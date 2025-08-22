pub struct RenamedAttrOpaque2;

impl Drop for RenamedAttrOpaque2 {
    fn drop(&mut self) {
        unsafe { namespace_AttrOpaque2_destroy(self) }
    }
}

impl RenamedAttrOpaque2 {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_AttrOpaque2_destroy(this : *mut RenamedAttrOpaque2);
}