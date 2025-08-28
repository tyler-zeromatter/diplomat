use super::OpaqueThin;
use super::OpaqueThinIter;
pub struct OpaqueThinVec;

impl Drop for OpaqueThinVec {
    fn drop(&mut self) {
        unsafe { OpaqueThinVec_destroy(self) }
    }
}

impl OpaqueThinVec {
    pub fn create<'anon_0, 'anon_1, 'anon_2>(a : &'anon_0 [i32], b : &'anon_1 [f32], c : &'anon_2 [u8]) -> Box<OpaqueThinVec> {
        let ret = unsafe { OpaqueThinVec_create(a, b, c.into()) };
        
        ret
    
    }

    pub fn iter<'a>(&'a self) -> Box<OpaqueThinIter<'a>> {
        let ret = unsafe { OpaqueThinVec_iter(self) };
        
        ret
    
    }

    pub fn len<'anon_0>(&'anon_0 self) -> usize {
        let ret = unsafe { OpaqueThinVec_len(self) };
        
        ret
    
    }

    pub fn get<'a>(&'a self, idx : usize) -> &'a Option<OpaqueThin> {
        let ret = unsafe { OpaqueThinVec_get(self, idx) };
        
        ret
    
    }

    pub fn first<'a>(&'a self) -> &'a Option<OpaqueThin> {
        let ret = unsafe { OpaqueThinVec_first(self) };
        
        ret
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OpaqueThinVec_create<'anon_0, 'anon_1, 'anon_2>(a : &'anon_0 [i32], b : &'anon_1 [f32], c : diplomat_runtime::DiplomatStrSlice<'anon_2>) -> Box<OpaqueThinVec>;

    fn OpaqueThinVec_iter<'a>(this: &'a OpaqueThinVec) -> Box<OpaqueThinIter<'a>>;

    fn OpaqueThinVec_len<'anon_0>(this: &'anon_0 OpaqueThinVec) -> usize;

    fn OpaqueThinVec_get<'a>(this: &'a OpaqueThinVec, idx : usize) -> &'a Option<OpaqueThin>;

    fn OpaqueThinVec_first<'a>(this: &'a OpaqueThinVec) -> &'a Option<OpaqueThin>;

    fn OpaqueThinVec_destroy(this : *mut OpaqueThinVec);
}