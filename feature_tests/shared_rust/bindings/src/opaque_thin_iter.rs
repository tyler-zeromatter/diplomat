use super::OpaqueThin;
pub struct OpaqueThinIter;

impl Drop for OpaqueThinIter {
    fn drop(&mut self) {
        unsafe { OpaqueThinIter_destroy(self) }
    }
}

impl OpaqueThinIter {
    pub fn next(&mut self) -> Option<&OpaqueThin> {
        let ret = unsafe { OpaqueThinIter_next(self) };
        ret
    }

    

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OpaqueThinIter_next(this: &mut OpaqueThinIter) -> Option<&OpaqueThin>;

    fn OpaqueThinIter_destroy(this : *mut OpaqueThinIter);

}