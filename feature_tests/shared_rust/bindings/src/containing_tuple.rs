pub struct ContainingTuple<'a>;

#[repr(C)]
pub(crate) struct ContainingTupleAbi<'a>;

impl<'a> ContainingTupleAbi<'a> {
    pub(crate) fn from_ffi(self) -> ContainingTuple<'a> {
        ContainingTuple {
            
        }
    }

    pub(crate) fn to_ffi(_this : ContainingTuple<'a>) -> ContainingTupleAbi<'a> {
        ContainingTupleAbi {
            
        }
    }
}

impl<'a> From<ContainingTuple<'a>> for ContainingTupleAbi<'a>{
    fn from(value: ContainingTuple<'a>) -> Self {
        ContainingTupleAbi::to_ffi(value)
    }
}

impl<'a> From<ContainingTupleAbi<'a>> for ContainingTuple<'a>{
    fn from(value: ContainingTupleAbi<'a>) -> Self {
        ContainingTupleAbi::from_ffi(value)
    }
}

impl<'a> ContainingTuple<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}