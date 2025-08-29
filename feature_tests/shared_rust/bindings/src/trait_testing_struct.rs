pub struct TraitTestingStruct;

#[repr(C)]
pub(crate) struct TraitTestingStructAbi;

impl TraitTestingStructAbi {
    fn from_ffi(self) -> TraitTestingStruct{
        TraitTestingStruct {
            
        }
    }
}

impl TraitTestingStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}