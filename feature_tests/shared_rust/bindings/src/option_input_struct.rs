use super::OptionEnum;
#[repr(C)]
pub struct OptionInputStruct {
    pub a: Option<u8>,
    pub b: Option<char>,
    pub c: Option<OptionEnum>,
}

impl OptionInputStruct {
}

#[link(name = "somelib")]
unsafe extern "C" {
}