use super::OpaqueThin;
use super::OpaqueThinIter;
pub struct OpaqueThinVec;

impl Drop for OpaqueThinVec {
    fn drop(&mut self) {
        unsafe { OpaqueThinVec_destroy(self) }
    }
}

impl OpaqueThinVec {
    pub fn create(a : &'anon_0 [i32], b : &'anon_1 [f32], c : &'anon_2 [u8]) -> Box<OpaqueThinVec> {
        let ret = unsafe { OpaqueThinVec_create(a, b, c.into()) };
        ret
    }

    pub fn iter(&self) -> Box<OpaqueThinIter><'a> {
        let ret = unsafe { OpaqueThinVec_iter(self) };
        ret
    }

    pub fn len(&self) -> usize {
        let ret = unsafe { OpaqueThinVec_len(self) };
        ret
    }

    pub fn get(&self, idx : usize) -> &'a Option<OpaqueThin> {
        let ret = unsafe { OpaqueThinVec_get(self, idx) };
        ret
    }

    pub fn first(&self) -> &'a Option<OpaqueThin> {
        let ret = unsafe { OpaqueThinVec_first(self) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OpaqueThinVec_create(a : &'anon_0 [i32], b : &'anon_1 [f32], c : &'anon_2 diplomat_runtime::DiplomatStrSlice) -> Box<OpaqueThinVec>;

    fn OpaqueThinVec_iter(this: &OpaqueThinVec) -> Box<OpaqueThinIter><'a>;

    fn OpaqueThinVec_len(this: &OpaqueThinVec) -> usize;

    fn OpaqueThinVec_get(this: &OpaqueThinVec, idx : usize) -> &'a Option<OpaqueThin>;

    fn OpaqueThinVec_first(this: &OpaqueThinVec) -> &'a Option<OpaqueThin>;

    fn OpaqueThinVec_destroy(this : *mut OpaqueThinVec);
}