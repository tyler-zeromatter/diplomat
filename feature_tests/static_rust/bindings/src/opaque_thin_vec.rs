use super::OpaqueThin;
use super::OpaqueThinIter;
pub struct OpaqueThinVec;

impl OpaqueThinVec {
    fn create(a : &[i32], b : &[f32], c : &[TODO]) -> OpaqueThinVec {
            // TODO: writeable conversions.
        unsafe { OpaqueThinVec_create(a, b, c) }
    }

    fn iter(&self) -> OpaqueThinIter {
            // TODO: writeable conversions.
        unsafe { OpaqueThinVec_iter(self) }
    }

    fn len(&self) -> usize {
            // TODO: writeable conversions.
        unsafe { OpaqueThinVec_len(self) }
    }

    fn get(&self, idx : usize) -> OpaqueThin {
            // TODO: writeable conversions.
        unsafe { OpaqueThinVec_get(self, idx) }
    }

    fn first(&self) -> OpaqueThin {
            // TODO: writeable conversions.
        unsafe { OpaqueThinVec_first(self) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OpaqueThinVec_create(a : &[i32], b : &[f32], c : &[TODO]) -> OpaqueThinVec;

    fn OpaqueThinVec_iter(this: &OpaqueThinVec) -> OpaqueThinIter;

    fn OpaqueThinVec_len(this: &OpaqueThinVec) -> usize;

    fn OpaqueThinVec_get(this: &OpaqueThinVec, idx : usize) -> OpaqueThin;

    fn OpaqueThinVec_first(this: &OpaqueThinVec) -> OpaqueThin;

}