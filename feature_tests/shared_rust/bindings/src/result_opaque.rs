use super::ErrorEnum;
use super::ErrorStruct;
pub struct ResultOpaque;

impl Drop for ResultOpaque {
    fn drop(&mut self) {
        unsafe { ResultOpaque_destroy(self) }
    }
}

impl ResultOpaque {
    pub fn new(i : i32) -> Result<Box<ResultOpaque>, ErrorEnum> {
        let ret = unsafe { ResultOpaque_new(i) };
        ret.into()
    }

    pub fn new_failing_foo() -> Result<Box<ResultOpaque>, ErrorEnum> {
        let ret = unsafe { ResultOpaque_new_failing_foo() };
        ret.into()
    }

    pub fn new_failing_bar() -> Result<Box<ResultOpaque>, ErrorEnum> {
        let ret = unsafe { ResultOpaque_new_failing_bar() };
        ret.into()
    }

    pub fn new_failing_unit() -> Result<Box<ResultOpaque>, ()> {
        let ret = unsafe { ResultOpaque_new_failing_unit() };
        ret.into()
    }

    pub fn new_failing_struct(i : i32) -> Result<Box<ResultOpaque>, ErrorStruct> {
        let ret = unsafe { ResultOpaque_new_failing_struct(i) };
        ret.into()
    }

    pub fn new_in_err(i : i32) -> Result<(), Box<ResultOpaque>> {
        let ret = unsafe { ResultOpaque_new_in_err(i) };
        ret.into()
    }

    pub fn new_int(i : i32) -> Result<i32, ()> {
        let ret = unsafe { ResultOpaque_new_int(i) };
        ret.into()
    }

    pub fn new_in_enum_err(i : i32) -> Result<ErrorEnum, Box<ResultOpaque>> {
        let ret = unsafe { ResultOpaque_new_in_enum_err(i) };
        ret.into()
    }

    pub fn takes_str(&mut self, _v : &String) -> &ResultOpaque {
        let ret = unsafe { ResultOpaque_takes_str(self, _v.into()) };
        ret
    }

    pub fn assert_integer(&self, i : i32) {
        let ret = unsafe { ResultOpaque_assert_integer(self, i) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn ResultOpaque_new(i : i32) -> crate::DiplomatResult<Box<ResultOpaque>, ErrorEnum>;

    fn ResultOpaque_new_failing_foo() -> crate::DiplomatResult<Box<ResultOpaque>, ErrorEnum>;

    fn ResultOpaque_new_failing_bar() -> crate::DiplomatResult<Box<ResultOpaque>, ErrorEnum>;

    fn ResultOpaque_new_failing_unit() -> crate::DiplomatResult<Box<ResultOpaque>, ()>;

    fn ResultOpaque_new_failing_struct(i : i32) -> crate::DiplomatResult<Box<ResultOpaque>, ErrorStruct>;

    fn ResultOpaque_new_in_err(i : i32) -> crate::DiplomatResult<(), Box<ResultOpaque>>;

    fn ResultOpaque_new_int(i : i32) -> crate::DiplomatResult<i32, ()>;

    fn ResultOpaque_new_in_enum_err(i : i32) -> crate::DiplomatResult<ErrorEnum, Box<ResultOpaque>>;

    fn ResultOpaque_takes_str(this: &mut ResultOpaque, _v : diplomat_runtime::DiplomatStrSlice) -> &ResultOpaque;

    fn ResultOpaque_assert_integer(this: &ResultOpaque, i : i32);

    fn ResultOpaque_destroy(this : *mut ResultOpaque);
}