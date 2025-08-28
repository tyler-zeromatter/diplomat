use super::ErrorEnum;
use super::ErrorStruct;
pub struct ResultOpaque;

impl Drop for ResultOpaque {
    fn drop(&mut self) {
        unsafe { ResultOpaque_destroy(self) }
    }
}

impl ResultOpaque {
    pub fn new(i : i32) -> Result<ResultOpaque, ErrorEnum> {
        let ret = unsafe { ResultOpaque_new(i) };
        
        ret.into()
    }

    pub fn new_failing_foo() -> Result<ResultOpaque, ErrorEnum> {
        let ret = unsafe { ResultOpaque_new_failing_foo() };
        
        ret.into()
    }

    pub fn new_failing_bar() -> Result<ResultOpaque, ErrorEnum> {
        let ret = unsafe { ResultOpaque_new_failing_bar() };
        
        ret.into()
    }

    pub fn new_failing_unit() -> Result<ResultOpaque, ()> {
        let ret = unsafe { ResultOpaque_new_failing_unit() };
        
        ret.into()
    }

    pub fn new_failing_struct(i : i32) -> Result<ResultOpaque, ErrorStruct> {
        let ret = unsafe { ResultOpaque_new_failing_struct(i) };
        
        ret.into()
    }

    pub fn new_in_err(i : i32) -> Result<(), ResultOpaque> {
        let ret = unsafe { ResultOpaque_new_in_err(i) };
        
        ret.into()
    }

    pub fn new_int(i : i32) -> Result<i32, ()> {
        let ret = unsafe { ResultOpaque_new_int(i) };
        
        ret.into()
    }

    pub fn new_in_enum_err(i : i32) -> Result<ErrorEnum, ResultOpaque> {
        let ret = unsafe { ResultOpaque_new_in_enum_err(i) };
        
        ret.into()
    }

    pub fn takes_str<'a, 'anon_0>(&'a mut self, _v : &'anon_0 String) -> &'a mut ResultOpaque {
        let ret = unsafe { ResultOpaque_takes_str(self, &_v.into()) };
        
        ret
    }

    pub fn assert_integer<'anon_0>(&'anon_0 self, i : i32) {
        let ret = unsafe { ResultOpaque_assert_integer(self, i) };
        }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn ResultOpaque_new(i : i32) -> crate::DiplomatResult<ResultOpaque, ErrorEnum>;

    fn ResultOpaque_new_failing_foo() -> crate::DiplomatResult<ResultOpaque, ErrorEnum>;

    fn ResultOpaque_new_failing_bar() -> crate::DiplomatResult<ResultOpaque, ErrorEnum>;

    fn ResultOpaque_new_failing_unit() -> crate::DiplomatResult<ResultOpaque, ()>;

    fn ResultOpaque_new_failing_struct(i : i32) -> crate::DiplomatResult<ResultOpaque, ErrorStruct>;

    fn ResultOpaque_new_in_err(i : i32) -> crate::DiplomatResult<(), ResultOpaque>;

    fn ResultOpaque_new_int(i : i32) -> crate::DiplomatResult<i32, ()>;

    fn ResultOpaque_new_in_enum_err(i : i32) -> crate::DiplomatResult<ErrorEnum, ResultOpaque>;

    fn ResultOpaque_takes_str<'a, 'anon_0>(this: &'a mut ResultOpaque, _v : &'anon_0 diplomat_runtime::DiplomatStrSlice) -> &'a mut ResultOpaque;

    fn ResultOpaque_assert_integer<'anon_0>(this: &'anon_0 ResultOpaque, i : i32);

    fn ResultOpaque_destroy(this : *mut ResultOpaque);
}