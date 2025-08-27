use super::OptionOpaque;
use super::OptionOpaqueChar;
#[repr(C)]
pub(super) struct OptionStruct {
    pub a: Box<Option<OptionOpaque>>,
    pub b: Box<Option<OptionOpaqueChar>>,
    pub c: u32,
    pub d: Box<OptionOpaque>,
}

impl OptionStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}