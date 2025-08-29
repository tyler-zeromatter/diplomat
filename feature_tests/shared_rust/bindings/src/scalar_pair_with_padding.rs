pub struct ScalarPairWithPadding;

#[repr(C)]
pub(crate) struct ScalarPairWithPaddingAbi;

impl ScalarPairWithPaddingAbi {
    pub(crate) fn from_ffi(self) -> ScalarPairWithPadding{
        ScalarPairWithPadding {
            
        }
    }

    pub (crate) fn to_ffi(this : ScalarPairWithPadding) -> ScalarPairWithPaddingAbi{
        ScalarPairWithPaddingAbi {
            
        }
    }
}

impl From<ScalarPairWithPadding> for ScalarPairWithPaddingAbi{
    fn from(value: ScalarPairWithPadding) -> Self {
        ScalarPairWithPaddingAbi::to_ffi(value)
    }
}

impl From<ScalarPairWithPaddingAbi> for ScalarPairWithPadding{
    fn from(value: ScalarPairWithPaddingAbi) -> Self {
        value.from_ffi()
    }
}

impl ScalarPairWithPadding {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}