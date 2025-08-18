#[diplomat_static_rust::bridge(lib_name = "somelib")]
pub mod ffi {
    use diplomat_runtime::{DiplomatChar, DiplomatOption, DiplomatWrite};
    #[repr(C)]
    pub struct OptionOpaque(i32);
    #[repr(C)]
    pub struct OptionOpaqueChar(char);
    #[repr(C)]
    pub struct OptionString(String);
    impl OptionString {
        pub fn new<'a>(diplomat_str: &'a DiplomatStr) -> Option<Box<Self>> {
            unsafe { OptionString_new(diplomat_str) }
        }
        pub fn write<'a>(&'a self, write: &'a mut DiplomatWrite) -> Result<(), ()> {
            unsafe { OptionString_write(self, write) }
        }
        pub fn borrow<'a>(&'a self) -> Option<&'a DiplomatStr> {
            unsafe { OptionString_borrow(self) }
        }
    }
    #[repr(C)]
    pub struct OptionStruct {
        a: Option<Box<OptionOpaque>>,
        b: Option<Box<OptionOpaqueChar>>,
        c: u32,
        d: Box<OptionOpaque>,
    }
    #[derive(Debug)]
    #[repr(C)]
    pub struct OptionInputStruct {
        a: DiplomatOption<u8>,
        b: DiplomatOption<DiplomatChar>,
        c: DiplomatOption<OptionEnum>,
    }
    impl OptionInputStruct {
        pub fn default_ctor() -> Self {
            unsafe { OptionInputStruct_default_ctor() }
        }
    }
    #[derive(Debug)]
    #[repr(C)]
    pub enum OptionEnum {
        Foo,
        Bar,
    }
    impl OptionOpaque {
        pub fn new(i: i32) -> Option<Box<OptionOpaque>> {
            unsafe { OptionString_new(i) }
        }
        pub fn new_none() -> Option<Box<OptionOpaque>> {
            unsafe { OptionOpaque_new_none() }
        }
        pub fn returns() -> Option<OptionStruct> {
            unsafe { OptionOpaque_returns() }
        }
        pub fn option_isize(&self) -> Option<isize> {
            unsafe { OptionOpaque_option_isize(self) }
        }
        pub fn option_usize(&self) -> Option<usize> {
            unsafe { OptionOpaque_option_usize(self) }
        }
        pub fn option_i32(&self) -> Option<i32> {
            unsafe { OptionOpaque_option_i32(self) }
        }
        pub fn option_u32(&self) -> Option<u32> {
            unsafe { OptionOpaque_option_u32(self) }
        }
        pub fn new_struct() -> OptionStruct {
            unsafe { OptionOpaque_new_struct() }
        }
        pub fn new_struct_nones() -> OptionStruct {
            unsafe { OptionOpaque_new_struct_nones() }
        }
        pub fn returns_none_self<'a>(&'a self) -> Option<&'a OptionOpaque> {
            unsafe { OptionOpaque_returns_none_self(self) }
        }
        pub fn returns_some_self<'a>(&'a self) -> Option<&'a OptionOpaque> {
            unsafe { OptionOpaque_returns_some_self(self) }
        }
        pub fn assert_integer(&self, i: i32) {
            unsafe { OptionOpaque_assert_integer(self, i) }
        }
        pub fn option_opaque_argument(arg: Option<&OptionOpaque>) -> bool {
            unsafe { OptionOpaque_option_opaque_argument(arg) }
        }
        pub fn accepts_option_u8(arg: Option<u8>, sentinel: u8) -> Option<u8> {
            unsafe { OptionOpaque_accepts_option_u8(arg, sentinel) }
        }
        pub fn accepts_option_enum(
            arg: Option<OptionEnum>,
            sentinel: u8,
        ) -> Option<OptionEnum> {
            unsafe { OptionOpaque_accepts_option_enum(arg, sentinel) }
        }
        pub fn accepts_option_input_struct(
            arg: Option<OptionInputStruct>,
            sentinel: u8,
        ) -> Option<OptionInputStruct> {
            unsafe { OptionOpaque_accepts_option_input_struct(arg, sentinel) }
        }
        pub fn returns_option_input_struct() -> OptionInputStruct {
            unsafe { OptionOpaque_returns_option_input_struct() }
        }
        pub fn accepts_option_str(arg: Option<&str>, sentinel: u8) -> usize {
            unsafe { OptionOpaque_accepts_option_str(arg, sentinel) }
        }
        pub fn accepts_option_str_slice(
            arg: Option<&[DiplomatStrSlice]>,
            sentinel: u8,
        ) -> bool {
            unsafe { OptionOpaque_accepts_option_str_slice(arg, sentinel) }
        }
        pub fn accepts_option_primitive(arg: Option<&[u32]>, sentinel: u8) -> i64 {
            unsafe { OptionOpaque_accepts_option_primitive(arg, sentinel) }
        }
    }
    impl OptionOpaqueChar {
        pub fn assert_char(&self, ch: DiplomatChar) {
            unsafe { OptionOpaqueChar_assert_char(self, ch) }
        }
    }
}
