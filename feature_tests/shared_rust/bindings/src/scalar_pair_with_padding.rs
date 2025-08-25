#[repr(C)]
pub struct ScalarPairWithPadding;

impl ScalarPairWithPadding {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}