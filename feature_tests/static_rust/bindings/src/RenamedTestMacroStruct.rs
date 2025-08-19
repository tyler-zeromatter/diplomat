#[repr(C)]
pub struct RenamedTestMacroStruct {

}

impl RenamedTestMacroStruct {
    fn test_func() {
        unsafe { namespace_TestMacroStruct_test_func() }
    }

    fn test_meta() {
        unsafe { namespace_TestMacroStruct_test_meta() }
    }

}