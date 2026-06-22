pub struct TupleStruct<'a>;

#[repr(C)]
pub(crate) struct TupleStructAbi<'a>;

impl<'a> TupleStructAbi<'a> {
    pub(crate) fn from_ffi(self) -> TupleStruct<'a> {
        TupleStruct {
            
        }
    }

    pub(crate) fn to_ffi(_this : TupleStruct<'a>) -> TupleStructAbi<'a> {
        TupleStructAbi {
            
        }
    }
}

impl<'a> From<TupleStruct<'a>> for TupleStructAbi<'a>{
    fn from(value: TupleStruct<'a>) -> Self {
        TupleStructAbi::to_ffi(value)
    }
}

impl<'a> From<TupleStructAbi<'a>> for TupleStruct<'a>{
    fn from(value: TupleStructAbi<'a>) -> Self {
        TupleStructAbi::from_ffi(value)
    }
}

impl<'a> TupleStruct<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}