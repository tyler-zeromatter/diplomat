#[diplomat_static_rust::bridge]
pub mod ffi {
    #[test]
    #[test]
    pub struct ResultOpaque(i32);
    #[test]
    #[test]
    pub enum ErrorEnum {
        Foo,
        Bar,
    }
    #[test]
    #[test]
    pub struct ErrorStruct {
        i: i32,
        j: i32,
    }
    impl ResultOpaque {
        #[test]
        pub fn new(i: i32) -> Result<Box<ResultOpaque>, ErrorEnum> {
            Ok(Box::new(ResultOpaque(i)))
        }
        #[test]
        pub fn new_failing_foo() -> Result<Box<ResultOpaque>, ErrorEnum> {
            Err(ErrorEnum::Foo)
        }
        #[test]
        pub fn new_failing_bar() -> Result<Box<ResultOpaque>, ErrorEnum> {
            Err(ErrorEnum::Bar)
        }
        #[test]
        pub fn new_failing_unit() -> Result<Box<ResultOpaque>, ()> {
            Err(())
        }
        #[test]
        pub fn new_failing_struct(i: i32) -> Result<Box<ResultOpaque>, ErrorStruct> {
            Err(ErrorStruct { i, j: 12 })
        }
        pub fn new_in_err(i: i32) -> Result<(), Box<ResultOpaque>> {
            Err(Box::new(ResultOpaque(i)))
        }
        pub fn new_int(i: i32) -> Result<i32, ()> {
            Ok(i)
        }
        #[test]
        pub fn new_failing_int(i: i32) -> Result<(), i32> {
            Err(i)
        }
        pub fn new_in_enum_err(i: i32) -> Result<ErrorEnum, Box<ResultOpaque>> {
            Err(Box::new(ResultOpaque(i)))
        }
        #[test]
        #[test]
        pub fn takes_str<'a>(&'a mut self, _v: &str) -> &'a mut Self {
            self
        }
        pub fn assert_integer(&self, i: i32) {
            assert_eq!(i, self.0);
        }
    }
}
