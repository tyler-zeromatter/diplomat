#[repr(C)]
pub struct TraitTestingStruct;

impl TraitTestingStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}