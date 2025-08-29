use super::UnimportedEnum;
pub struct ImportedStruct {
    pub foo: UnimportedEnum,
    pub count: u8,
}

#[repr(C)]
pub(crate) struct ImportedStructAbi {
    
    foo : UnimportedEnum,
    
    count : u8,
    
}

impl ImportedStructAbi {
    fn from_ffi(self) -> ImportedStruct{
        ImportedStruct {
            
                foo: self.foo,
            
                count: self.count,
            
        }
    }
}

impl ImportedStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}