#[repr(C)]
pub struct TraitWrapper;

impl TraitWrapper {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}