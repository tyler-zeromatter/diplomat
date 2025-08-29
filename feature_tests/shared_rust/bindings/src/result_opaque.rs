use super::ErrorEnum;
use super::ErrorStruct;
use super::error_struct::ErrorStructAbi;
pub struct ResultOpaque;

impl Drop for ResultOpaque {
    fn drop(&mut self) {
        unsafe { ResultOpaque_destroy(self) }
    }
}

impl ResultOpaque {
    pub fn new(i : i32) -> Result<Box<ResultOpaque>, ErrorEnum> {
        let ret = unsafe { ResultOpaque_new(i) };
        
        ret.to_result()
    
    }

    pub fn new_failing_foo() -> Result<Box<ResultOpaque>, ErrorEnum> {
        let ret = unsafe { ResultOpaque_new_failing_foo() };
        
        ret.to_result()
    
    }

    pub fn new_failing_bar() -> Result<Box<ResultOpaque>, ErrorEnum> {
        let ret = unsafe { ResultOpaque_new_failing_bar() };
        
        ret.to_result()
    
    }

    pub fn new_failing_unit() -> Result<Box<ResultOpaque>, ()> {
        let ret = unsafe { ResultOpaque_new_failing_unit() };
        
        ret.to_result()
    
    }

    pub fn new_failing_struct(i : i32) -> Result<Box<ResultOpaque>, ErrorStruct> {
        let ret = unsafe { ResultOpaque_new_failing_struct(i) };
        
        ret.to_result().map_err(|err : ErrorStructAbi| { err.from_ffi() })
    
    }

    pub fn new_in_err(i : i32) -> Result<(), Box<ResultOpaque>> {
        let ret = unsafe { ResultOpaque_new_in_err(i) };
        
        ret.to_result()
    
    }

    pub fn new_int(i : i32) -> Result<i32, ()> {
        let ret = unsafe { ResultOpaque_new_int(i) };
        
        ret.to_result()
    
    }

    pub fn new_in_enum_err(i : i32) -> Result<ErrorEnum, Box<ResultOpaque>> {
        let ret = unsafe { ResultOpaque_new_in_enum_err(i) };
        
        ret.to_result()
    
    }

    pub fn takes_str<'a, 'anon_0>(&'a mut self, _v : &'anon_0 str) -> &'a mut ResultOpaque {
        let ret = unsafe { ResultOpaque_takes_str(self, _v.as_bytes().into()) };
        
        ret
    
    }

    pub fn assert_integer<'anon_0>(&'anon_0 self, i : i32) {
        unsafe { ResultOpaque_assert_integer(self, i) };
        
    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn ResultOpaque_new(i : i32) -> crate::DiplomatResult<Box<ResultOpaque>, ErrorEnum>;

    fn ResultOpaque_new_failing_foo() -> crate::DiplomatResult<Box<ResultOpaque>, ErrorEnum>;

    fn ResultOpaque_new_failing_bar() -> crate::DiplomatResult<Box<ResultOpaque>, ErrorEnum>;

    fn ResultOpaque_new_failing_unit() -> crate::DiplomatResult<Box<ResultOpaque>, ()>;

    fn ResultOpaque_new_failing_struct(i : i32) -> crate::DiplomatResult<Box<ResultOpaque>, ErrorStructAbi>;

    fn ResultOpaque_new_in_err(i : i32) -> crate::DiplomatResult<(), Box<ResultOpaque>>;

    fn ResultOpaque_new_int(i : i32) -> crate::DiplomatResult<i32, ()>;

    fn ResultOpaque_new_in_enum_err(i : i32) -> crate::DiplomatResult<ErrorEnum, Box<ResultOpaque>>;

    fn ResultOpaque_takes_str<'a, 'anon_0>(this: &'a mut ResultOpaque, _v : diplomat_runtime::DiplomatStrSlice<'anon_0>) -> &'a mut ResultOpaque;

    fn ResultOpaque_assert_integer<'anon_0>(this: &'anon_0 ResultOpaque, i : i32);

    fn ResultOpaque_destroy(this : *mut ResultOpaque);
}