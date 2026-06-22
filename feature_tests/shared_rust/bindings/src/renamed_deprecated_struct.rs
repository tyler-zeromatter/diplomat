#[repr(C)]
pub struct RenamedDeprecatedStruct;

impl RenamedDeprecatedStruct {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}