#[repr(C)]
pub struct RenamedTestMacroStruct {
    pub a: usize,
}

impl RenamedTestMacroStruct {
    pub fn test_func() -> usize {
        let ret = unsafe { namespace_TestMacroStruct_test_func() };
        ret
    }

    pub fn test_meta() -> RenamedTestMacroStruct {
        let ret = unsafe { namespace_TestMacroStruct_test_meta() };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_TestMacroStruct_test_func() -> usize;

    fn namespace_TestMacroStruct_test_meta() -> RenamedTestMacroStruct;

}