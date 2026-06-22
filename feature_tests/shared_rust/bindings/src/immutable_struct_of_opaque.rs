#[repr(C)]
pub struct ImmutableStructOfOpaque<'a>;

impl<'a> ImmutableStructOfOpaque<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}