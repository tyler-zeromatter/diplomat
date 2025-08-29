pub struct TraitWrapper;

#[repr(C)]
pub(crate) struct TraitWrapperAbi;

impl TraitWrapperAbi {
    pub(crate) fn from_ffi(self) -> TraitWrapper{
        TraitWrapper {
            
        }
    }

    pub (crate) fn to_ffi(_this : TraitWrapper) -> TraitWrapperAbi{
        TraitWrapperAbi {
            
        }
    }
}

impl From<TraitWrapper> for TraitWrapperAbi{
    fn from(value: TraitWrapper) -> Self {
        TraitWrapperAbi::to_ffi(value)
    }
}

impl From<TraitWrapperAbi> for TraitWrapper{
    fn from(value: TraitWrapperAbi) -> Self {
        TraitWrapperAbi::from_ffi(value)
    }
}

impl TraitWrapper {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}