#[repr(C)]
pub struct OutTupleStruct;

impl OutTupleStruct {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}