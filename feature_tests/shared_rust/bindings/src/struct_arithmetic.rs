#[repr(C)]
pub struct StructArithmetic;

impl StructArithmetic {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}