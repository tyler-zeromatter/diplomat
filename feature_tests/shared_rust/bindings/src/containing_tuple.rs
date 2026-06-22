#[repr(C)]
pub struct ContainingTuple<'a>;

impl<'a> ContainingTuple<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}