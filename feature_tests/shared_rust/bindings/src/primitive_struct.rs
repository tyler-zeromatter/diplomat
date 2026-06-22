#[repr(C)]
pub struct PrimitiveStruct {
    pub x: f32,
    pub a: bool,
    pub b: char,
    pub c: i64,
    pub d: isize,
    pub e: u8,
}

impl PrimitiveStruct {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}