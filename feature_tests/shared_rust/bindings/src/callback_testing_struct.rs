pub struct CallbackTestingStruct;

#[repr(C)]
pub(crate) struct CallbackTestingStructAbi;

impl CallbackTestingStructAbi {
    fn from_ffi(self) -> CallbackTestingStruct{
        CallbackTestingStruct {
            
        }
    }
}

impl CallbackTestingStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}