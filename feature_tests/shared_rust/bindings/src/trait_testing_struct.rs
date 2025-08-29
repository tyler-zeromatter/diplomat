pub struct TraitTestingStruct;

#[repr(C)]
pub(crate) struct TraitTestingStructAbi;

impl TraitTestingStructAbi {
    pub(crate) fn from_ffi(self) -> TraitTestingStruct{
        TraitTestingStruct {
            
        }
    }

    pub (crate) fn to_ffi(this : TraitTestingStruct) -> TraitTestingStructAbi{
        TraitTestingStructAbi {
            
        }
    }
}

impl From<TraitTestingStruct> for TraitTestingStructAbi{
    fn from(value: TraitTestingStruct) -> Self {
        TraitTestingStructAbi::to_ffi(value)
    }
}

impl From<TraitTestingStructAbi> for TraitTestingStruct{
    fn from(value: TraitTestingStructAbi) -> Self {
        TraitTestingStructAbi::from_ffi(value)
    }
}

impl TraitTestingStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}