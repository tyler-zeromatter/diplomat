use super::OpaqueThin;
use super::OpaqueThinIter;
pub struct OpaqueThinVec;

impl OpaqueThinVec {
    pub fn create(a : &[i32], b : &[f32], c : &[TODO]) -> Box<OpaqueThinVec> {
            // TODO: writeable conversions.
        unsafe { OpaqueThinVec_create(a, b, c) }
    }

    pub fn iter(&self) -> Box<OpaqueThinIter> {
            // TODO: writeable conversions.
        unsafe { OpaqueThinVec_iter(self) }
    }

    pub fn len(&self) -> usize {
            // TODO: writeable conversions.
        unsafe { OpaqueThinVec_len(self) }
    }

    pub fn get(&self, idx : usize) -> &Option<OpaqueThin> {
            // TODO: writeable conversions.
        unsafe { OpaqueThinVec_get(self, idx) }
    }

    pub fn first(&self) -> &Option<OpaqueThin> {
            // TODO: writeable conversions.
        unsafe { OpaqueThinVec_first(self) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OpaqueThinVec_create(a : &[i32], b : &[f32], c : &[TODO]) -> Box<OpaqueThinVec>;

    fn OpaqueThinVec_iter(this: &OpaqueThinVec) -> Box<OpaqueThinIter>;

    fn OpaqueThinVec_len(this: &OpaqueThinVec) -> usize;

    fn OpaqueThinVec_get(this: &OpaqueThinVec, idx : usize) -> &Option<OpaqueThin>;

    fn OpaqueThinVec_first(this: &OpaqueThinVec) -> &Option<OpaqueThin>;

}