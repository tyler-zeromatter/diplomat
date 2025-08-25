#[repr(C)]
pub struct BigStructWithStuff;

impl BigStructWithStuff {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}