pub struct OpaqueThin;

impl Drop for OpaqueThin {
    fn drop(&mut self) {
        unsafe { OpaqueThin_destroy(self) }
    }
}

impl OpaqueThin {
    pub fn a(&self) -> i32 {
        let ret = unsafe { OpaqueThin_a(self) };
        ret
    }

    pub fn b(&self) -> f32 {
        let ret = unsafe { OpaqueThin_b(self) };
        ret
    }

    pub fn c(&self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { OpaqueThin_c(self, write_mut) };
        let out_str = write.to_string();
        out_str
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OpaqueThin_a(this: &OpaqueThin) -> i32;

    fn OpaqueThin_b(this: &OpaqueThin) -> f32;

    fn OpaqueThin_c(this: &OpaqueThin, write_mut : &mut crate::DiplomatWrite) -> ();

    fn OpaqueThin_destroy(this : *mut OpaqueThin);
}