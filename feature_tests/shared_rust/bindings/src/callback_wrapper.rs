pub struct CallbackWrapper;

#[repr(C)]
pub(crate) struct CallbackWrapperAbi;

impl CallbackWrapperAbi {
    pub(crate) fn from_ffi(self) -> CallbackWrapper {
        CallbackWrapper {
            
        }
    }

    pub(crate) fn to_ffi(_this : CallbackWrapper) -> CallbackWrapperAbi {
        CallbackWrapperAbi {
            
        }
    }
}

impl From<CallbackWrapper> for CallbackWrapperAbi{
    fn from(value: CallbackWrapper) -> Self {
        CallbackWrapperAbi::to_ffi(value)
    }
}

impl From<CallbackWrapperAbi> for CallbackWrapper{
    fn from(value: CallbackWrapperAbi) -> Self {
        CallbackWrapperAbi::from_ffi(value)
    }
}

impl CallbackWrapper {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}