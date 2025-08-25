use super::OptionEnum;
#[repr(C)]
pub struct OptionInputStruct {
    pub a: Option<u8>,
    pub b: Option<diplomat_runtime::DiplomatChar>,
    pub c: Option<OptionEnum>,
}

impl OptionInputStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}