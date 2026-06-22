#[repr(C)]
pub struct TupleStruct<'a>;

impl<'a> TupleStruct<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}