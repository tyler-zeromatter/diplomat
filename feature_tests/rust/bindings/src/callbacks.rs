#[diplomat_static_rust::bridge]
mod ffi {
    use crate::slices::ffi::MyString;
    pub struct CallbackWrapper {
        cant_be_empty: bool,
    }
    pub struct CallbackTestingStruct {
        x: i32,
        y: i32,
    }
    impl CallbackWrapper {
        pub fn test_multi_arg_callback(f: impl Fn(i32) -> i32, x: i32) -> i32 {
            unsafe { CallbackWrapper_test_multi_arg_callback(f, x) }
        }
        pub fn test_no_args(h: impl Fn()) -> i32 {
            unsafe { CallbackWrapper_test_no_args(h) }
        }
        pub fn test_cb_with_struct(f: impl Fn(CallbackTestingStruct) -> i32) -> i32 {
            unsafe { CallbackWrapper_test_cb_with_struct(f) }
        }
        pub fn test_multiple_cb_args(
            f: impl Fn() -> i32,
            g: impl Fn(i32) -> i32,
        ) -> i32 {
            unsafe { CallbackWrapper_test_multiple_cb_args(f, g) }
        }
        pub fn test_str_cb_arg(f: impl Fn(&str) -> i32) -> i32 {
            unsafe { CallbackWrapper_test_str_cb_arg(f) }
        }
        pub fn test_opaque_cb_arg<'a>(cb: impl Fn(&mut MyString), a: &'a mut MyString) {
            unsafe { CallbackWrapper_test_opaque_cb_arg(cb, a) }
        }
        pub fn test_slice_cb_arg(arg: &[u8], f: impl Fn(&[u8])) {
            unsafe { CallbackWrapper_test_slice_cb_arg(arg, f) }
        }
        pub fn test_result_output(t: impl Fn() -> Result<(), ()>) {
            unsafe { CallbackWrapper_test_result_output(t) }
        }
        pub fn test_result_usize_output(t: impl Fn() -> Result<usize, ()>) {
            unsafe { CallbackWrapper_test_result_usize_output(t) }
        }
        pub fn test_option_output(t: impl Fn() -> Option<()>) {
            unsafe { CallbackWrapper_test_option_output(t) }
        }
        pub fn test_diplomat_option_output(t: impl Fn() -> DiplomatOption<u32>) {
            unsafe { CallbackWrapper_test_diplomat_option_output(t) }
        }
        pub fn test_option_opaque<'a>(
            t: impl Fn() -> Option<&'a crate::structs::ffi::Opaque>,
            w: &mut DiplomatWrite,
        ) {
            unsafe { CallbackWrapper_test_option_opaque(t, w) }
        }
        pub fn test_diplomat_result(t: impl Fn() -> DiplomatResult<usize, usize>) {
            unsafe { CallbackWrapper_test_diplomat_result(t) }
        }
        pub fn test_result_opaque<'a>(
            t: impl Fn() -> Result<&'a crate::structs::ffi::Opaque, ()>,
            w: &mut DiplomatWrite,
        ) {
            unsafe { CallbackWrapper_test_result_opaque(t, w) }
        }
        pub fn test_inner_conversion(
            t: impl Fn(
            ) -> Result<crate::structs::ffi::MyStructContainingAnOption, usize>,
        ) {
            unsafe { CallbackWrapper_test_inner_conversion(t) }
        }
        pub fn test_str_conversion<'a>(
            t: impl Fn() -> Result<DiplomatStrSlice<'a>, ()>,
        ) {
            unsafe { CallbackWrapper_test_str_conversion(t) }
        }
        pub fn test_slice_conversion<'a>(t: impl Fn() -> Result<&'a [f64], ()>) {
            unsafe { CallbackWrapper_test_slice_conversion(t) }
        }
        pub fn test_struct_slice_conversion<'a>(
            t: impl Fn() -> Result<&'a [crate::structs::ffi::PrimitiveStruct], ()>,
        ) {
            unsafe { CallbackWrapper_test_struct_slice_conversion(t) }
        }
        pub fn test_opaque_result_error<'a>(
            t: impl Fn() -> Result<(), &'a crate::structs::ffi::Opaque>,
            w: &mut DiplomatWrite,
        ) {
            unsafe { CallbackWrapper_test_opaque_result_error(t, w) }
        }
    }
    pub struct CallbackHolder {
        held: Box<dyn Fn(i32) -> i32>,
    }
    impl CallbackHolder {
        pub fn new(func: impl Fn(i32) -> i32 + 'static) -> Box<Self> {
            unsafe { MutableCallbackHolder_new(func) }
        }
        pub fn call(&self, a: i32) -> i32 {
            unsafe { MutableCallbackHolder_call(self, a) }
        }
    }
    pub struct MutableCallbackHolder {
        held: Box<dyn FnMut(i32) -> i32>,
    }
    impl MutableCallbackHolder {
        pub fn new(func: impl FnMut(i32) -> i32 + 'static) -> Box<Self> {
            unsafe { MutableCallbackHolder_new(func) }
        }
        pub fn call(&mut self, a: i32) -> i32 {
            unsafe { MutableCallbackHolder_call(self, a) }
        }
    }
}
