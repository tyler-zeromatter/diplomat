use super::OpaqueThin;
use std::marker::PhantomData;

pub struct OpaqueThinIter<'a> {
    a_marker : PhantomData<&'a ()>,
}

impl<'a> Drop for OpaqueThinIter<'a> {
    fn drop(&mut self) {
        unsafe { OpaqueThinIter_destroy(self) }
    }
}

impl<'a> OpaqueThinIter<'a> {
    pub fn next(&'a mut self) -> &'a Option<OpaqueThin> {
        let ret = unsafe { OpaqueThinIter_next(self) };
        
        ret
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OpaqueThinIter_next<'a>(this: &'a mut OpaqueThinIter) -> &'a Option<OpaqueThin>;

    fn OpaqueThinIter_destroy(this : *mut OpaqueThinIter);
}