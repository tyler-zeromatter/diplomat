pub struct CallbackTestingStruct;

#[repr(C)]
pub(crate) struct CallbackTestingStructAbi;

impl CallbackTestingStructAbi {
    pub(crate) fn from_ffi(self) -> CallbackTestingStruct{
        CallbackTestingStruct {
            
        }
    }

    pub (crate) fn to_ffi(this : CallbackTestingStruct) -> CallbackTestingStructAbi{
        CallbackTestingStructAbi {
            
        }
    }
}

impl From<CallbackTestingStruct> for CallbackTestingStructAbi{
    fn from(value: CallbackTestingStruct) -> Self {
        CallbackTestingStructAbi::to_ffi(value)
    }
}

impl From<CallbackTestingStructAbi> for CallbackTestingStruct{
    fn from(value: CallbackTestingStructAbi) -> Self {
        CallbackTestingStructAbi::from_ffi(value)
    }
}

impl CallbackTestingStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}