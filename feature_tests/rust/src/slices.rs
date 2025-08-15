#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatStrSlice, DiplomatWrite};
    use std::fmt::Write as _;
    #[test]
    pub struct MyString(String);
    impl MyString {
        #[test]
        pub fn new(v: &DiplomatStr) -> Box<MyString> {
            Box::new(Self(String::from_utf8(v.to_owned()).unwrap()))
        }
        #[test]
        pub fn new_unsafe(v: &str) -> Box<MyString> {
            Box::new(Self(v.to_string()))
        }
        pub fn new_owned(v: Box<DiplomatStr>) -> Box<MyString> {
            Box::new(Self(String::from_utf8(v.into()).unwrap()))
        }
        pub fn new_from_first(v: &[DiplomatStrSlice]) -> Box<MyString> {
            Box::new(Self(core::str::from_utf8(v[0].into()).unwrap().into()))
        }
        #[test]
        pub fn set_str(&mut self, new_str: &DiplomatStr) {
            self.0 = String::from_utf8(new_str.to_owned()).unwrap();
        }
        #[test]
        pub fn get_str(&self, write: &mut DiplomatWrite) {
            let _infallible = write!(write, "{}", self.0);
        }
        pub fn get_static_str() -> &'static str {
            "hello"
        }
        pub fn string_transform(foo: &str, write: &mut DiplomatWrite) {
            let _ = foo;
            let _ = write;
        }
        pub fn borrow<'a>(&'a self) -> DiplomatStrSlice<'a> {
            AsRef::<[u8]>::as_ref(&self.0).into()
        }
    }
    #[test]
    struct Float64Vec(Vec<f64>);
    impl Float64Vec {
        #[test]
        pub fn new(v: &[f64]) -> Box<Float64Vec> {
            Box::new(Self(v.to_vec()))
        }
        #[test]
        pub fn new_bool(v: &[bool]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as u8 as f64).collect()))
        }
        #[test]
        pub fn new_i16(v: &[i16]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }
        #[test]
        pub fn new_u16(v: &[u16]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }
        #[test]
        pub fn new_isize(v: &[isize]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }
        #[test]
        pub fn new_usize(v: &[usize]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }
        #[test]
        pub fn new_f64_be_bytes(v: &[DiplomatByte]) -> Box<Float64Vec> {
            Box::new(Self(
                v.chunks_exact(8)
                    .map(|b| f64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
                    .collect(),
            ))
        }
        #[test]
        #[test]
        pub fn new_from_owned(v: Box<[f64]>) -> Box<Float64Vec> {
            Box::new(Self(v.into()))
        }
        #[test]
        pub fn as_slice<'a>(&'a self) -> &'a [f64] {
            &self.0
        }
        pub fn fill_slice(&self, v: &mut [f64]) {
            v.copy_from_slice(&self.0)
        }
        pub fn set_value(&mut self, new_slice: &[f64]) {
            self.0 = new_slice.to_vec();
        }
        #[test]
        pub fn to_string(&self, w: &mut DiplomatWrite) {
            let _infallible = write!(w, "{:?}", self.0);
        }
        #[test]
        pub fn borrow<'a>(&'a self) -> &'a [f64] {
            &self.0
        }
        #[test]
        pub fn get(&self, i: usize) -> Option<f64> {
            self.0.get(i).copied()
        }
    }
    #[test]
    #[test]
    struct Float64VecError(Vec<f64>);
    impl Float64VecError {
        #[test]
        pub fn new(v: &[f64]) -> Box<Float64VecError> {
            Box::new(Self(v.to_vec()))
        }
        #[test]
        pub fn get(&self, i: usize) -> Result<f64, ()> {
            if let Some(i) = self.0.get(i) {
                Ok(*i)
            } else {
                Err(())
            }
        }
    }
}
