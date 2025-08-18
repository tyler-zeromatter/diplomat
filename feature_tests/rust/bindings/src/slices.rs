#[diplomat_static_rust::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatStrSlice, DiplomatWrite};
    use std::fmt::Write as _;
    pub struct MyString(String);
    impl MyString {
        pub fn new(v: &DiplomatStr) -> Box<MyString> {
            unsafe {}
        }
        pub fn new_unsafe(v: &str) -> Box<MyString> {
            unsafe {}
        }
        pub fn new_owned(v: Box<DiplomatStr>) -> Box<MyString> {
            unsafe {}
        }
        pub fn new_from_first(v: &[DiplomatStrSlice]) -> Box<MyString> {
            unsafe {}
        }
        pub fn set_str(&mut self, new_str: &DiplomatStr) {
            unsafe {}
        }
        pub fn get_str(&self, write: &mut DiplomatWrite) {
            unsafe {}
        }
        pub fn get_static_str() -> &'static str {
            unsafe {}
        }
        pub fn string_transform(foo: &str, write: &mut DiplomatWrite) {
            unsafe {}
        }
        pub fn borrow<'a>(&'a self) -> DiplomatStrSlice<'a> {
            unsafe {}
        }
    }
    struct Float64Vec(Vec<f64>);
    impl Float64Vec {
        pub fn new(v: &[f64]) -> Box<Float64Vec> {
            unsafe {}
        }
        pub fn new_bool(v: &[bool]) -> Box<Float64Vec> {
            unsafe {}
        }
        pub fn new_i16(v: &[i16]) -> Box<Float64Vec> {
            unsafe {}
        }
        pub fn new_u16(v: &[u16]) -> Box<Float64Vec> {
            unsafe {}
        }
        pub fn new_isize(v: &[isize]) -> Box<Float64Vec> {
            unsafe {}
        }
        pub fn new_usize(v: &[usize]) -> Box<Float64Vec> {
            unsafe {}
        }
        pub fn new_f64_be_bytes(v: &[DiplomatByte]) -> Box<Float64Vec> {
            unsafe {}
        }
        pub fn new_from_owned(v: Box<[f64]>) -> Box<Float64Vec> {
            unsafe {}
        }
        pub fn as_slice<'a>(&'a self) -> &'a [f64] {
            unsafe {}
        }
        pub fn fill_slice(&self, v: &mut [f64]) {
            unsafe {}
        }
        pub fn set_value(&mut self, new_slice: &[f64]) {
            unsafe {}
        }
        pub fn to_string(&self, w: &mut DiplomatWrite) {
            unsafe {}
        }
        #[allow(clippy::needless_lifetimes)]
        pub fn borrow<'a>(&'a self) -> &'a [f64] {
            unsafe {}
        }
        pub fn get(&self, i: usize) -> Option<f64> {
            unsafe {}
        }
    }
    struct Float64VecError(Vec<f64>);
    impl Float64VecError {
        pub fn new(v: &[f64]) -> Box<Float64VecError> {
            unsafe {}
        }
        pub fn get(&self, i: usize) -> Result<f64, ()> {
            unsafe {}
        }
    }
}
