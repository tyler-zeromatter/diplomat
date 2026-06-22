pub struct OutTupleStruct;

#[repr(C)]
pub(crate) struct OutTupleStructAbi;

impl OutTupleStructAbi {
    pub(crate) fn from_ffi(self) -> OutTupleStruct {
        OutTupleStruct {
            
        }
    }

    pub(crate) fn to_ffi(_this : OutTupleStruct) -> OutTupleStructAbi {
        OutTupleStructAbi {
            
        }
    }
}

impl From<OutTupleStruct> for OutTupleStructAbi{
    fn from(value: OutTupleStruct) -> Self {
        OutTupleStructAbi::to_ffi(value)
    }
}

impl From<OutTupleStructAbi> for OutTupleStruct{
    fn from(value: OutTupleStructAbi) -> Self {
        OutTupleStructAbi::from_ffi(value)
    }
}

impl OutTupleStruct {

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}