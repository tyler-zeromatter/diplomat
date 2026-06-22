#[repr(C)]
pub struct RenamedFeatureTest;

impl RenamedFeatureTest {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}