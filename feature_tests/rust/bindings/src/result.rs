#[diplomat_static_rust::bridge]
pub mod ffi {
    pub struct ResultOpaque(i32);
    #[derive(PartialEq, Eq, Debug)]
    pub enum ErrorEnum {
        Foo,
        Bar,
    }
    #[derive(Debug)]
    pub struct ErrorStruct {
        i: i32,
        j: i32,
    }
    impl ResultOpaque {
        pub fn new(i: i32) -> Result<Box<ResultOpaque>, ErrorEnum> {
            unsafe { ResultOpaque_new(i) }
        }
        pub fn new_failing_foo() -> Result<Box<ResultOpaque>, ErrorEnum> {
            unsafe { ResultOpaque_new_failing_foo() }
        }
        pub fn new_failing_bar() -> Result<Box<ResultOpaque>, ErrorEnum> {
            unsafe { ResultOpaque_new_failing_bar() }
        }
        pub fn new_failing_unit() -> Result<Box<ResultOpaque>, ()> {
            unsafe { ResultOpaque_new_failing_unit() }
        }
        pub fn new_failing_struct(i: i32) -> Result<Box<ResultOpaque>, ErrorStruct> {
            unsafe { ResultOpaque_new_failing_struct(i) }
        }
        pub fn new_in_err(i: i32) -> Result<(), Box<ResultOpaque>> {
            unsafe { ResultOpaque_new_in_err(i) }
        }
        pub fn new_int(i: i32) -> Result<i32, ()> {
            unsafe { ResultOpaque_new_int(i) }
        }
        pub fn new_failing_int(i: i32) -> Result<(), i32> {
            unsafe { ResultOpaque_new_failing_int(i) }
        }
        pub fn new_in_enum_err(i: i32) -> Result<ErrorEnum, Box<ResultOpaque>> {
            unsafe { ResultOpaque_new_in_enum_err(i) }
        }
        /// When we take &str, the return type becomes a Result
        /// Test that this interacts gracefully with returning a reference type
        pub fn takes_str<'a>(&'a mut self, _v: &str) -> &'a mut Self {
            unsafe { ResultOpaque_takes_str(self, _v) }
        }
        pub fn assert_integer(&self, i: i32) {
            unsafe { ResultOpaque_assert_integer(self, i) }
        }
    }
}
