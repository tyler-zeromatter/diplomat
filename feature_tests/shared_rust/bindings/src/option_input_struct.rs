use super::OptionEnum;
#[repr(C)]
pub struct OptionInputStruct {
    pub a: diplomat_runtime::DiplomatOption::<u8>,
    pub b: diplomat_runtime::DiplomatOption::<char>,
    pub c: diplomat_runtime::DiplomatOption::<OptionEnum>,
}

impl OptionInputStruct {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}