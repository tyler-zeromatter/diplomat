#[repr(C)]
pub struct RenamedTestMacroStruct {
    pub a: usize,
}

impl RenamedTestMacroStruct {
    pub fn test_func() -> usize {
            // TODO: writeable conversions.
        unsafe { namespace_TestMacroStruct_test_func() }
    }

    pub fn test_meta() -> RenamedTestMacroStruct {
            // TODO: writeable conversions.
        unsafe { namespace_TestMacroStruct_test_meta() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_TestMacroStruct_test_func() -> usize;

    fn namespace_TestMacroStruct_test_meta() -> RenamedTestMacroStruct;

}