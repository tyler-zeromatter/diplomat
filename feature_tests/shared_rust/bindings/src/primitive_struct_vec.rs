pub struct PrimitiveStructVec;

impl Drop for PrimitiveStructVec {
    fn drop(&mut self) {
        unsafe { PrimitiveStructVec_destroy(self) }
    }
}

impl PrimitiveStructVec {
    

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn PrimitiveStructVec_destroy(this : *mut PrimitiveStructVec);

}