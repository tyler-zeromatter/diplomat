pub struct FallibleOpaqueConstructor;

#[repr(C)]
pub(crate) struct FallibleOpaqueConstructorAbi;

impl FallibleOpaqueConstructorAbi {
    pub(crate) fn from_ffi(self) -> FallibleOpaqueConstructor {
        FallibleOpaqueConstructor {
            
        }
    }

    pub(crate) fn to_ffi(_this : FallibleOpaqueConstructor) -> FallibleOpaqueConstructorAbi {
        FallibleOpaqueConstructorAbi {
            
        }
    }
}

impl From<FallibleOpaqueConstructor> for FallibleOpaqueConstructorAbi{
    fn from(value: FallibleOpaqueConstructor) -> Self {
        FallibleOpaqueConstructorAbi::to_ffi(value)
    }
}

impl From<FallibleOpaqueConstructorAbi> for FallibleOpaqueConstructor{
    fn from(value: FallibleOpaqueConstructorAbi) -> Self {
        FallibleOpaqueConstructorAbi::from_ffi(value)
    }
}

impl FallibleOpaqueConstructor {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}