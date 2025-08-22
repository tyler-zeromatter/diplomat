#[repr(C)]
pub struct TraitTestingStruct;

impl TraitTestingStruct {
}

#[link(name = "somelib")]
unsafe extern "C" {}