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
    pub(crate) fn from_ffi(self) -> ImportedStruct{
        ImportedStruct {
            
            foo: self.foo,
            
            count: self.count,
            
        }
    }

    pub (crate) fn to_ffi(this : ImportedStruct) -> ImportedStructAbi{
        ImportedStructAbi {
            
            foo : this.foo,
            
            count : this.count,
            
        }
    }
}

impl From<ImportedStruct> for ImportedStructAbi{
    fn from(value: ImportedStruct) -> Self {
        ImportedStructAbi::to_ffi(value)
    }
}

impl From<ImportedStructAbi> for ImportedStruct{
    fn from(value: ImportedStructAbi) -> Self {
        value.from_ffi()
    }
}

impl ImportedStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}