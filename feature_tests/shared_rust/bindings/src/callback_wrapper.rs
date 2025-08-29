pub struct CallbackWrapper;

#[repr(C)]
pub(crate) struct CallbackWrapperAbi;

impl CallbackWrapperAbi {
    fn from_ffi(self) -> CallbackWrapper{
        CallbackWrapper {
            
        }
    }
}

impl CallbackWrapper {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}