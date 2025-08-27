#[repr(C)]
pub struct RenamedTestMacroStruct {
    pub a: usize,
}

impl RenamedTestMacroStruct {
    pub fn test_func<'a>() -> usize {
        let ret = unsafe { namespace_TestMacroStruct_test_func() };
        ret
    }

    pub fn test_meta() -> RenamedTestMacroStruct {
        let ret = unsafe { namespace_TestMacroStruct_test_meta() };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_TestMacroStruct_test_func<'a>() -> usize;

    fn namespace_TestMacroStruct_test_meta() -> RenamedTestMacroStruct;
}