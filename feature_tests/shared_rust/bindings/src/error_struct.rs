#[repr(C)]
pub struct ErrorStruct {
    pub i: i32,
    pub j: i32,
}

impl ErrorStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}