#[repr(C)]
pub struct RenamedPartialComparableSlice;

impl RenamedPartialComparableSlice {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}