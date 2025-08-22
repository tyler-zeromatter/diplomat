pub struct RenamedAttrOpaque2;

impl Drop for RenamedAttrOpaque2 {
    fn drop(&mut self) {
        unsafe { RenamedAttrOpaque2_destroy(self) }
    }
}

impl RenamedAttrOpaque2 {
    

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RenamedAttrOpaque2_destroy(this : *mut RenamedAttrOpaque2);

}