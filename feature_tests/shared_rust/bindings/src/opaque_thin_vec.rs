use super::OpaqueThin;
use super::OpaqueThinIter;
pub struct OpaqueThinVec;

impl Drop for OpaqueThinVec {
    fn drop(&mut self) {
        unsafe { OpaqueThinVec_destroy(self) }
    }
}

impl OpaqueThinVec {
    pub fn create(a : [i32], b : [f32], c : [u8]) -> Box<OpaqueThinVec> {
        let ret = unsafe { OpaqueThinVec_create(a, b, c.into()) };
        ret
    }

    pub fn iter(&self) -> Box<OpaqueThinIter> {
        let ret = unsafe { OpaqueThinVec_iter(self) };
        ret
    }

    pub fn len(&self) -> usize {
        let ret = unsafe { OpaqueThinVec_len(self) };
        ret
    }

    pub fn get(&self, idx : usize) -> Option<OpaqueThin> {
        let ret = unsafe { OpaqueThinVec_get(self, idx) };
        ret
    }

    pub fn first(&self) -> Option<OpaqueThin> {
        let ret = unsafe { OpaqueThinVec_first(self) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OpaqueThinVec_create(a : [i32], b : [f32], c : diplomat_runtime::DiplomatStrSlice) -> Box<OpaqueThinVec>;

    fn OpaqueThinVec_iter(this: &OpaqueThinVec) -> Box<OpaqueThinIter>;

    fn OpaqueThinVec_len(this: &OpaqueThinVec) -> usize;

    fn OpaqueThinVec_get(this: &OpaqueThinVec, idx : usize) -> Option<OpaqueThin>;

    fn OpaqueThinVec_first(this: &OpaqueThinVec) -> Option<OpaqueThin>;

    fn OpaqueThinVec_destroy(this : *mut OpaqueThinVec);
}