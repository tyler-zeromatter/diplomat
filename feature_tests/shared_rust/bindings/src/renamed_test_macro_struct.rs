pub struct RenamedTestMacroStruct {
    pub a: usize,
}

#[repr(C)]
pub(crate) struct RenamedTestMacroStructAbi {
    a : usize,
}

impl RenamedTestMacroStructAbi {
    pub(crate) fn from_ffi(self) -> RenamedTestMacroStruct {
        RenamedTestMacroStruct {
            
            a: self.a,
            
        }
    }

    pub(crate) fn to_ffi(this : RenamedTestMacroStruct) -> RenamedTestMacroStructAbi {
        RenamedTestMacroStructAbi {
            
            a : this.a,
            
        }
    }
}

impl From<RenamedTestMacroStruct> for RenamedTestMacroStructAbi{
    fn from(value: RenamedTestMacroStruct) -> Self {
        RenamedTestMacroStructAbi::to_ffi(value)
    }
}

impl From<RenamedTestMacroStructAbi> for RenamedTestMacroStruct{
    fn from(value: RenamedTestMacroStructAbi) -> Self {
        RenamedTestMacroStructAbi::from_ffi(value)
    }
}

impl RenamedTestMacroStruct {
    pub fn test_func<'a>() -> usize {
        let ret = unsafe { namespace_TestMacroStruct_test_func() };
        
        ret
    
    }

    pub fn test_meta() -> RenamedTestMacroStruct {
        let ret = unsafe { namespace_TestMacroStruct_test_meta() };
        
        ret.from_ffi()
    
    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_TestMacroStruct_test_func<'a>() -> usize;

    fn namespace_TestMacroStruct_test_meta() -> RenamedTestMacroStructAbi;
}