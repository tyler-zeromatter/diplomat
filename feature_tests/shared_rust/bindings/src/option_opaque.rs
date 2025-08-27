use super::OptionEnum;
use super::OptionInputStruct;
use super::OptionStruct;
pub struct OptionOpaque;

impl Drop for OptionOpaque {
    fn drop(&mut self) {
        unsafe { OptionOpaque_destroy(self) }
    }
}

impl OptionOpaque {
    pub fn new(i : i32) -> Option<Box<OptionOpaque>> {
        let ret = unsafe { OptionOpaque_new(i) };
        ret
    }

    pub fn new_none() -> Option<Box<OptionOpaque>> {
        let ret = unsafe { OptionOpaque_new_none() };
        ret
    }

    pub fn returns() -> Option<OptionStruct> {
        let ret = unsafe { OptionOpaque_returns() };
        ret.into_converted_option()
    }

    pub fn option_isize(&self) -> Option<isize> {
        let ret = unsafe { OptionOpaque_option_isize(self) };
        ret.into_converted_option()
    }

    pub fn option_usize(&self) -> Option<usize> {
        let ret = unsafe { OptionOpaque_option_usize(self) };
        ret.into_converted_option()
    }

    pub fn option_i32(&self) -> Option<i32> {
        let ret = unsafe { OptionOpaque_option_i32(self) };
        ret.into_converted_option()
    }

    pub fn option_u32(&self) -> Option<u32> {
        let ret = unsafe { OptionOpaque_option_u32(self) };
        ret.into_converted_option()
    }

    pub fn new_struct() -> OptionStruct {
        let ret = unsafe { OptionOpaque_new_struct() };
        ret
    }

    pub fn new_struct_nones() -> OptionStruct {
        let ret = unsafe { OptionOpaque_new_struct_nones() };
        ret
    }

    pub fn returns_none_self(&self) -> Option<OptionOpaque> {
        let ret = unsafe { OptionOpaque_returns_none_self(self) };
        ret
    }

    pub fn returns_some_self(&self) -> Option<OptionOpaque> {
        let ret = unsafe { OptionOpaque_returns_some_self(self) };
        ret
    }

    pub fn assert_integer(&self, i : i32) {
        let ret = unsafe { OptionOpaque_assert_integer(self, i) };
        ret
    }

    pub fn option_opaque_argument(arg : Option<OptionOpaque>) -> bool {
        let ret = unsafe { OptionOpaque_option_opaque_argument(arg) };
        ret
    }

    pub fn accepts_option_u8(arg : Option<u8>, sentinel : u8) -> Option<u8> {
        let ret = unsafe { OptionOpaque_accepts_option_u8(arg, sentinel) };
        ret.into_converted_option()
    }

    pub fn accepts_option_enum(arg : Option<OptionEnum>, sentinel : u8) -> Option<OptionEnum> {
        let ret = unsafe { OptionOpaque_accepts_option_enum(arg, sentinel) };
        ret.into_converted_option()
    }

    pub fn accepts_option_input_struct(arg : Option<OptionInputStruct>, sentinel : u8) -> Option<OptionInputStruct> {
        let ret = unsafe { OptionOpaque_accepts_option_input_struct(arg, sentinel) };
        ret.into_converted_option()
    }

    pub fn returns_option_input_struct() -> OptionInputStruct {
        let ret = unsafe { OptionOpaque_returns_option_input_struct() };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OptionOpaque_new(i : i32) -> Option<Box<OptionOpaque>>;

    fn OptionOpaque_new_none() -> Option<Box<OptionOpaque>>;

    fn OptionOpaque_returns() -> diplomat_runtime::DiplomatOption<OptionStruct>;

    fn OptionOpaque_option_isize(this: &OptionOpaque) -> diplomat_runtime::DiplomatOption<isize>;

    fn OptionOpaque_option_usize(this: &OptionOpaque) -> diplomat_runtime::DiplomatOption<usize>;

    fn OptionOpaque_option_i32(this: &OptionOpaque) -> diplomat_runtime::DiplomatOption<i32>;

    fn OptionOpaque_option_u32(this: &OptionOpaque) -> diplomat_runtime::DiplomatOption<u32>;

    fn OptionOpaque_new_struct() -> OptionStruct;

    fn OptionOpaque_new_struct_nones() -> OptionStruct;

    fn OptionOpaque_returns_none_self(this: &OptionOpaque) -> Option<OptionOpaque>;

    fn OptionOpaque_returns_some_self(this: &OptionOpaque) -> Option<OptionOpaque>;

    fn OptionOpaque_assert_integer(this: &OptionOpaque, i : i32);

    fn OptionOpaque_option_opaque_argument(arg : Option<OptionOpaque>) -> bool;

    fn OptionOpaque_accepts_option_u8(arg : diplomat_runtime::DiplomatOption<u8>, sentinel : u8) -> diplomat_runtime::DiplomatOption<u8>;

    fn OptionOpaque_accepts_option_enum(arg : diplomat_runtime::DiplomatOption<OptionEnum>, sentinel : u8) -> diplomat_runtime::DiplomatOption<OptionEnum>;

    fn OptionOpaque_accepts_option_input_struct(arg : diplomat_runtime::DiplomatOption<OptionInputStruct>, sentinel : u8) -> diplomat_runtime::DiplomatOption<OptionInputStruct>;

    fn OptionOpaque_returns_option_input_struct() -> OptionInputStruct;

    fn OptionOpaque_destroy(this : *mut OptionOpaque);
}