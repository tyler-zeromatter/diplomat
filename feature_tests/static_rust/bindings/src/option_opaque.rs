use super::OptionEnum;
use super::OptionInputStruct;
use super::OptionStruct;
pub struct OptionOpaque;

impl OptionOpaque {
    fn new(i : i32) -> OptionOpaque {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_new(i) }
    }

    fn new_none() -> OptionOpaque {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_new_none() }
    }

    fn returns() -> Option<OptionStruct> {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_returns() }
    }

    fn option_isize(&self) -> Option<isize> {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_option_isize(self) }
    }

    fn option_usize(&self) -> Option<usize> {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_option_usize(self) }
    }

    fn option_i32(&self) -> Option<i32> {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_option_i32(self) }
    }

    fn option_u32(&self) -> Option<u32> {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_option_u32(self) }
    }

    fn new_struct() -> OptionStruct {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_new_struct() }
    }

    fn new_struct_nones() -> OptionStruct {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_new_struct_nones() }
    }

    fn returns_none_self(&self) -> OptionOpaque {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_returns_none_self(self) }
    }

    fn returns_some_self(&self) -> OptionOpaque {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_returns_some_self(self) }
    }

    fn assert_integer(&self, i : i32) {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_assert_integer(self, i) }
    }

    fn option_opaque_argument(arg : OptionOpaque) -> bool {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_option_opaque_argument(arg) }
    }

    fn accepts_option_u8(arg : Option<u8>, sentinel : u8) -> Option<u8> {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_accepts_option_u8(arg, sentinel) }
    }

    fn accepts_option_enum(arg : Option<OptionEnum>, sentinel : u8) -> Option<OptionEnum> {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_accepts_option_enum(arg, sentinel) }
    }

    fn accepts_option_input_struct(arg : Option<OptionInputStruct>, sentinel : u8) -> Option<OptionInputStruct> {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_accepts_option_input_struct(arg, sentinel) }
    }

    fn returns_option_input_struct() -> OptionInputStruct {
            // TODO: writeable conversions.
        unsafe { OptionOpaque_returns_option_input_struct() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OptionOpaque_new(i : i32) -> OptionOpaque;

    fn OptionOpaque_new_none() -> OptionOpaque;

    fn OptionOpaque_returns() -> Option<OptionStruct>;

    fn OptionOpaque_option_isize(this: &OptionOpaque) -> Option<isize>;

    fn OptionOpaque_option_usize(this: &OptionOpaque) -> Option<usize>;

    fn OptionOpaque_option_i32(this: &OptionOpaque) -> Option<i32>;

    fn OptionOpaque_option_u32(this: &OptionOpaque) -> Option<u32>;

    fn OptionOpaque_new_struct() -> OptionStruct;

    fn OptionOpaque_new_struct_nones() -> OptionStruct;

    fn OptionOpaque_returns_none_self(this: &OptionOpaque) -> OptionOpaque;

    fn OptionOpaque_returns_some_self(this: &OptionOpaque) -> OptionOpaque;

    fn OptionOpaque_assert_integer(this: &OptionOpaque, i : i32);

    fn OptionOpaque_option_opaque_argument(arg : OptionOpaque) -> bool;

    fn OptionOpaque_accepts_option_u8(arg : DiplomatOption<Option<u8>>, sentinel : u8) -> Option<u8>;

    fn OptionOpaque_accepts_option_enum(arg : DiplomatOption<Option<OptionEnum>>, sentinel : u8) -> Option<OptionEnum>;

    fn OptionOpaque_accepts_option_input_struct(arg : DiplomatOption<Option<OptionInputStruct>>, sentinel : u8) -> Option<OptionInputStruct>;

    fn OptionOpaque_returns_option_input_struct() -> OptionInputStruct;

}