use super::ErrorEnum;
use super::ErrorStruct;
pub struct ResultOpaque;

impl ResultOpaque {
    pub fn new(i : i32) -> Result<Box<ResultOpaque>, ErrorEnum> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new(i) }
    }

    pub fn new_failing_foo() -> Result<Box<ResultOpaque>, ErrorEnum> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_failing_foo() }
    }

    pub fn new_failing_bar() -> Result<Box<ResultOpaque>, ErrorEnum> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_failing_bar() }
    }

    pub fn new_failing_unit() -> Result<Box<ResultOpaque>, ()> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_failing_unit() }
    }

    pub fn new_failing_struct(i : i32) -> Result<Box<ResultOpaque>, ErrorStruct> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_failing_struct(i) }
    }

    pub fn new_in_err(i : i32) -> Result<(), Box<ResultOpaque>> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_in_err(i) }
    }

    pub fn new_int(i : i32) -> Result<i32, ()> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_int(i) }
    }

    pub fn new_in_enum_err(i : i32) -> Result<ErrorEnum, Box<ResultOpaque>> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_in_enum_err(i) }
    }

    pub fn takes_str(&mut self, _v : &[TODO]) -> &ResultOpaque {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_takes_str(self, _v) }
    }

    pub fn assert_integer(&self, i : i32) {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_assert_integer(self, i) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn ResultOpaque_new(i : i32) -> DiplomatResult<Box<ResultOpaque>, ErrorEnum>;

    fn ResultOpaque_new_failing_foo() -> DiplomatResult<Box<ResultOpaque>, ErrorEnum>;

    fn ResultOpaque_new_failing_bar() -> DiplomatResult<Box<ResultOpaque>, ErrorEnum>;

    fn ResultOpaque_new_failing_unit() -> DiplomatResult<Box<ResultOpaque>, ()>;

    fn ResultOpaque_new_failing_struct(i : i32) -> DiplomatResult<Box<ResultOpaque>, ErrorStruct>;

    fn ResultOpaque_new_in_err(i : i32) -> DiplomatResult<(), Box<ResultOpaque>>;

    fn ResultOpaque_new_int(i : i32) -> DiplomatResult<i32, ()>;

    fn ResultOpaque_new_in_enum_err(i : i32) -> DiplomatResult<ErrorEnum, Box<ResultOpaque>>;

    fn ResultOpaque_takes_str(this: &mut ResultOpaque, _v : &[TODO]) -> &ResultOpaque;

    fn ResultOpaque_assert_integer(this: &ResultOpaque, i : i32);

}