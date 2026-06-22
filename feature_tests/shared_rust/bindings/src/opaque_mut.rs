pub struct OpaqueMut;

impl Drop for OpaqueMut {
    fn drop(&mut self) {
        unsafe { OpaqueMut_destroy(self) }
    }
}

impl OpaqueMut {
    pub fn new() -> Box<OpaqueMut> {
        let ret = unsafe { OpaqueMut_new() };
        
        ret

    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OpaqueMut_new() -> Box<OpaqueMut>;

    fn OpaqueMut_destroy(this : *mut OpaqueMut);
}