#[repr(C)]
pub struct TraitWrapper;

impl TraitWrapper {
}

#[link(name = "somelib")]
unsafe extern "C" {
}