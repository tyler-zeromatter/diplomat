pub struct TraitWrapper;

#[repr(C)]
pub(crate) struct TraitWrapperAbi;

impl TraitWrapperAbi {
    fn from_ffi(self) -> TraitWrapper{
        TraitWrapper {
            
        }
    }
}

impl TraitWrapper {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}