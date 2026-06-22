#[repr(C)]
pub struct FallibleOpaqueConstructor;

impl FallibleOpaqueConstructor {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}