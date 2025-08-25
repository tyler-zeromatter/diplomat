#[repr(C)]
pub struct MyZst;

impl MyZst {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}