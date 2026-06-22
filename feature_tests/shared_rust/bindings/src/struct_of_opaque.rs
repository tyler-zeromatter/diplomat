use super::Opaque;
use super::OpaqueMut;
#[repr(C)]
pub struct StructOfOpaque<'a> {
    pub i: &'a Opaque,
    pub j: &'a mut OpaqueMut,
}

impl<'a> StructOfOpaque<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}