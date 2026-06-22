pub struct RenamedMixinTest;

impl Drop for RenamedMixinTest {
    fn drop(&mut self) {
        unsafe { namespace_MixinTest_destroy(self) }
    }
}

impl RenamedMixinTest {
    pub fn hello() -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        unsafe { namespace_MixinTest_hello(write_mut) };
        
        let out_str = write.to_string();
        out_str

    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_MixinTest_hello(write_mut : &mut crate::DiplomatWrite) -> ();

    fn namespace_MixinTest_destroy(this : *mut RenamedMixinTest);
}