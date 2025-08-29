pub struct ScalarPairWithPadding;

#[repr(C)]
pub(crate) struct ScalarPairWithPaddingAbi;

impl ScalarPairWithPaddingAbi {
    fn from_ffi(self) -> ScalarPairWithPadding{
        ScalarPairWithPadding {
            
        }
    }
}

impl ScalarPairWithPadding {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}