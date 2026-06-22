pub struct RenamedDeprecatedStruct;

#[repr(C)]
pub(crate) struct RenamedDeprecatedStructAbi;

impl RenamedDeprecatedStructAbi {
    pub(crate) fn from_ffi(self) -> RenamedDeprecatedStruct {
        RenamedDeprecatedStruct {
            
        }
    }

    pub(crate) fn to_ffi(_this : RenamedDeprecatedStruct) -> RenamedDeprecatedStructAbi {
        RenamedDeprecatedStructAbi {
            
        }
    }
}

impl From<RenamedDeprecatedStruct> for RenamedDeprecatedStructAbi{
    fn from(value: RenamedDeprecatedStruct) -> Self {
        RenamedDeprecatedStructAbi::to_ffi(value)
    }
}

impl From<RenamedDeprecatedStructAbi> for RenamedDeprecatedStruct{
    fn from(value: RenamedDeprecatedStructAbi) -> Self {
        RenamedDeprecatedStructAbi::from_ffi(value)
    }
}

impl RenamedDeprecatedStruct {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}