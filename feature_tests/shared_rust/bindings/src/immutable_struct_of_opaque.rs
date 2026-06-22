pub struct ImmutableStructOfOpaque<'a>;

#[repr(C)]
pub(crate) struct ImmutableStructOfOpaqueAbi<'a>;

impl<'a> ImmutableStructOfOpaqueAbi<'a> {
    pub(crate) fn from_ffi(self) -> ImmutableStructOfOpaque<'a> {
        ImmutableStructOfOpaque {
            
        }
    }

    pub(crate) fn to_ffi(_this : ImmutableStructOfOpaque<'a>) -> ImmutableStructOfOpaqueAbi<'a> {
        ImmutableStructOfOpaqueAbi {
            
        }
    }
}

impl<'a> From<ImmutableStructOfOpaque<'a>> for ImmutableStructOfOpaqueAbi<'a>{
    fn from(value: ImmutableStructOfOpaque<'a>) -> Self {
        ImmutableStructOfOpaqueAbi::to_ffi(value)
    }
}

impl<'a> From<ImmutableStructOfOpaqueAbi<'a>> for ImmutableStructOfOpaque<'a>{
    fn from(value: ImmutableStructOfOpaqueAbi<'a>) -> Self {
        ImmutableStructOfOpaqueAbi::from_ffi(value)
    }
}

impl<'a> ImmutableStructOfOpaque<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}