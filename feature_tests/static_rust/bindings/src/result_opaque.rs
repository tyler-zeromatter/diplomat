use super::ErrorEnum;
use super::ErrorStruct;
pub struct ResultOpaque;

impl ResultOpaque {
    fn new(i : i32) -> Result<ResultOpaque, ErrorEnum> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new(i) }
    }

    fn new_failing_foo() -> Result<ResultOpaque, ErrorEnum> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_failing_foo() }
    }

    fn new_failing_bar() -> Result<ResultOpaque, ErrorEnum> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_failing_bar() }
    }

    fn new_failing_unit() -> Result<ResultOpaque, ()> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_failing_unit() }
    }

    fn new_failing_struct(i : i32) -> Result<ResultOpaque, ErrorStruct> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_failing_struct(i) }
    }

    fn new_in_err(i : i32) -> Result<(), ResultOpaque> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_in_err(i) }
    }

    fn new_int(i : i32) -> Result<i32, ()> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_int(i) }
    }

    fn new_in_enum_err(i : i32) -> Result<ErrorEnum, ResultOpaque> {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_new_in_enum_err(i) }
    }

    fn takes_str(&mut self, _v : TODO()) -> ResultOpaque {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_takes_str(self, _v) }
    }

    fn assert_integer(&self, i : i32) {
            // TODO: writeable conversions.
        unsafe { ResultOpaque_assert_integer(self, i) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn ResultOpaque_new(i : i32) -> Result<ResultOpaque, ErrorEnum>;

    fn ResultOpaque_new_failing_foo() -> Result<ResultOpaque, ErrorEnum>;

    fn ResultOpaque_new_failing_bar() -> Result<ResultOpaque, ErrorEnum>;

    fn ResultOpaque_new_failing_unit() -> Result<ResultOpaque, ()>;

    fn ResultOpaque_new_failing_struct(i : i32) -> Result<ResultOpaque, ErrorStruct>;

    fn ResultOpaque_new_in_err(i : i32) -> Result<(), ResultOpaque>;

    fn ResultOpaque_new_int(i : i32) -> Result<i32, ()>;

    fn ResultOpaque_new_in_enum_err(i : i32) -> Result<ErrorEnum, ResultOpaque>;

    fn ResultOpaque_takes_str(this: &mut ResultOpaque, _v : TODO()) -> ResultOpaque;

    fn ResultOpaque_assert_integer(this: &ResultOpaque, i : i32);

}