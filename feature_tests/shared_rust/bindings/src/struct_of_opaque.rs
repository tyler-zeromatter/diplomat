use super::Opaque;
use super::OpaqueMut;
pub struct StructOfOpaque<'a> {
    pub i: &'a Opaque,
    pub j: &'a mut OpaqueMut,
}

#[repr(C)]
pub(crate) struct StructOfOpaqueAbi<'a> {
    i : &'a Opaque,
    j : &'a mut OpaqueMut,
}

impl<'a> StructOfOpaqueAbi<'a> {
    pub(crate) fn from_ffi(self) -> StructOfOpaque<'a> {
        StructOfOpaque {
            
            i: self.i,
            
            j: self.j,
            
        }
    }

    pub(crate) fn to_ffi(this : StructOfOpaque<'a>) -> StructOfOpaqueAbi<'a> {
        StructOfOpaqueAbi {
            
            i : this.i,
            
            j : this.j,
            
        }
    }
}

impl<'a> From<StructOfOpaque<'a>> for StructOfOpaqueAbi<'a>{
    fn from(value: StructOfOpaque<'a>) -> Self {
        StructOfOpaqueAbi::to_ffi(value)
    }
}

impl<'a> From<StructOfOpaqueAbi<'a>> for StructOfOpaque<'a>{
    fn from(value: StructOfOpaqueAbi<'a>) -> Self {
        StructOfOpaqueAbi::from_ffi(value)
    }
}

impl<'a> StructOfOpaque<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}