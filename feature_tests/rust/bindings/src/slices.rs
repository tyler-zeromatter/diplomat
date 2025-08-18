#[diplomat_static_rust::bridge(lib_name = "somelib")]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatStrSlice, DiplomatWrite};
    use std::fmt::Write as _;
    pub struct MyString(String);
    impl MyString {
        pub fn new(v: &DiplomatStr) -> Box<MyString> {
            unsafe { MyString_new(v) }
        }
        pub fn new_unsafe(v: &str) -> Box<MyString> {
            unsafe { MyString_new_unsafe(v) }
        }
        pub fn new_owned(v: Box<DiplomatStr>) -> Box<MyString> {
            unsafe { MyString_new_owned(v) }
        }
        pub fn new_from_first(v: &[DiplomatStrSlice]) -> Box<MyString> {
            unsafe { MyString_new_from_first(v) }
        }
        pub fn set_str(&mut self, new_str: &DiplomatStr) {
            unsafe { MyString_set_str(self, new_str) }
        }
        pub fn get_str(&self, write: &mut DiplomatWrite) {
            unsafe { MyString_get_str(self, write) }
        }
        pub fn get_static_str() -> &'static str {
            unsafe { MyString_get_static_str() }
        }
        pub fn string_transform(foo: &str, write: &mut DiplomatWrite) {
            unsafe { MyString_string_transform(foo, write) }
        }
        pub fn borrow<'a>(&'a self) -> DiplomatStrSlice<'a> {
            unsafe { MyString_borrow(self) }
        }
    }
    struct Float64Vec(Vec<f64>);
    impl Float64Vec {
        pub fn new(v: &[f64]) -> Box<Float64Vec> {
            unsafe { MyString_new(v) }
        }
        pub fn new_bool(v: &[bool]) -> Box<Float64Vec> {
            unsafe { Float64Vec_new_bool(v) }
        }
        pub fn new_i16(v: &[i16]) -> Box<Float64Vec> {
            unsafe { Float64Vec_new_i16(v) }
        }
        pub fn new_u16(v: &[u16]) -> Box<Float64Vec> {
            unsafe { Float64Vec_new_u16(v) }
        }
        pub fn new_isize(v: &[isize]) -> Box<Float64Vec> {
            unsafe { Float64Vec_new_isize(v) }
        }
        pub fn new_usize(v: &[usize]) -> Box<Float64Vec> {
            unsafe { Float64Vec_new_usize(v) }
        }
        pub fn new_f64_be_bytes(v: &[DiplomatByte]) -> Box<Float64Vec> {
            unsafe { Float64Vec_new_f64_be_bytes(v) }
        }
        pub fn new_from_owned(v: Box<[f64]>) -> Box<Float64Vec> {
            unsafe { Float64Vec_new_from_owned(v) }
        }
        pub fn as_slice<'a>(&'a self) -> &'a [f64] {
            unsafe { Float64Vec_as_slice(self) }
        }
        pub fn fill_slice(&self, v: &mut [f64]) {
            unsafe { Float64Vec_fill_slice(self, v) }
        }
        pub fn set_value(&mut self, new_slice: &[f64]) {
            unsafe { Float64Vec_set_value(self, new_slice) }
        }
        pub fn to_string(&self, w: &mut DiplomatWrite) {
            unsafe { Float64Vec_to_string(self, w) }
        }
        #[allow(clippy::needless_lifetimes)]
        pub fn borrow<'a>(&'a self) -> &'a [f64] {
            unsafe { MyString_borrow(self) }
        }
        pub fn get(&self, i: usize) -> Option<f64> {
            unsafe { Float64VecError_get(self, i) }
        }
    }
    struct Float64VecError(Vec<f64>);
    impl Float64VecError {
        pub fn new(v: &[f64]) -> Box<Float64VecError> {
            unsafe { MyString_new(v) }
        }
        pub fn get(&self, i: usize) -> Result<f64, ()> {
            unsafe { Float64VecError_get(self, i) }
        }
    }
}
