use super::OpaqueThin;
pub struct OpaqueThinIter;

impl Drop for OpaqueThinIter {
    fn drop(&mut self) {
        unsafe { OpaqueThinIter_destroy(self) }
    }
}

impl OpaqueThinIter {
    pub fn next<'a>(&mut self) -> &'a Option<OpaqueThin> {
        let ret = unsafe { OpaqueThinIter_next(self) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OpaqueThinIter_next<'a>(this: &mut OpaqueThinIter) -> &'a Option<OpaqueThin>;

    fn OpaqueThinIter_destroy(this : *mut OpaqueThinIter);
}