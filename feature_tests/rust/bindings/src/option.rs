#[diplomat_static_rust::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatChar, DiplomatOption, DiplomatWrite};
    pub struct OptionOpaque(i32);
    pub struct OptionOpaqueChar(char);
    pub struct OptionString(String);
    impl OptionString {
        pub fn new<'a>(diplomat_str: &'a DiplomatStr) -> Option<Box<Self>> {
            unsafe {}
        }
        pub fn write<'a>(&'a self, write: &'a mut DiplomatWrite) -> Result<(), ()> {
            unsafe {}
        }
        pub fn borrow<'a>(&'a self) -> Option<&'a DiplomatStr> {
            unsafe {}
        }
    }
    pub struct OptionStruct {
        a: Option<Box<OptionOpaque>>,
        b: Option<Box<OptionOpaqueChar>>,
        c: u32,
        d: Box<OptionOpaque>,
    }
    #[derive(Debug)]
    pub struct OptionInputStruct {
        a: DiplomatOption<u8>,
        b: DiplomatOption<DiplomatChar>,
        c: DiplomatOption<OptionEnum>,
    }
    impl OptionInputStruct {
        pub fn default_ctor() -> Self {
            unsafe {}
        }
    }
    #[derive(Debug)]
    pub enum OptionEnum {
        Foo,
        Bar,
    }
    impl OptionOpaque {
        pub fn new(i: i32) -> Option<Box<OptionOpaque>> {
            unsafe {}
        }
        pub fn new_none() -> Option<Box<OptionOpaque>> {
            unsafe {}
        }
        pub fn returns() -> Option<OptionStruct> {
            unsafe {}
        }
        pub fn option_isize(&self) -> Option<isize> {
            unsafe {}
        }
        pub fn option_usize(&self) -> Option<usize> {
            unsafe {}
        }
        pub fn option_i32(&self) -> Option<i32> {
            unsafe {}
        }
        pub fn option_u32(&self) -> Option<u32> {
            unsafe {}
        }
        pub fn new_struct() -> OptionStruct {
            unsafe {}
        }
        pub fn new_struct_nones() -> OptionStruct {
            unsafe {}
        }
        pub fn returns_none_self<'a>(&'a self) -> Option<&'a OptionOpaque> {
            unsafe {}
        }
        pub fn returns_some_self<'a>(&'a self) -> Option<&'a OptionOpaque> {
            unsafe {}
        }
        pub fn assert_integer(&self, i: i32) {
            unsafe {}
        }
        pub fn option_opaque_argument(arg: Option<&OptionOpaque>) -> bool {
            unsafe {}
        }
        pub fn accepts_option_u8(arg: Option<u8>, sentinel: u8) -> Option<u8> {
            unsafe {}
        }
        pub fn accepts_option_enum(
            arg: Option<OptionEnum>,
            sentinel: u8,
        ) -> Option<OptionEnum> {
            unsafe {}
        }
        pub fn accepts_option_input_struct(
            arg: Option<OptionInputStruct>,
            sentinel: u8,
        ) -> Option<OptionInputStruct> {
            unsafe {}
        }
        pub fn returns_option_input_struct() -> OptionInputStruct {
            unsafe {}
        }
        pub fn accepts_option_str(arg: Option<&str>, sentinel: u8) -> usize {
            unsafe {}
        }
        pub fn accepts_option_str_slice(
            arg: Option<&[DiplomatStrSlice]>,
            sentinel: u8,
        ) -> bool {
            unsafe {}
        }
        pub fn accepts_option_primitive(arg: Option<&[u32]>, sentinel: u8) -> i64 {
            unsafe {}
        }
    }
    impl OptionOpaqueChar {
        pub fn assert_char(&self, ch: DiplomatChar) {
            unsafe {}
        }
    }
}
