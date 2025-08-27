pub struct Float64Vec;

impl Drop for Float64Vec {
    fn drop(&mut self) {
        unsafe { Float64Vec_destroy(self) }
    }
}

impl Float64Vec {
    pub fn new_bool<'anon_0>(v : &'anon_0 [bool]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_bool(v) };
        ret
    }

    pub fn new_i16<'anon_0>(v : &'anon_0 [i16]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_i16(v) };
        ret
    }

    pub fn new_u16<'anon_0>(v : &'anon_0 [u16]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_u16(v) };
        ret
    }

    pub fn new_isize<'anon_0>(v : &'anon_0 [isize]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_isize(v) };
        ret
    }

    pub fn new_usize<'anon_0>(v : &'anon_0 [usize]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_usize(v) };
        ret
    }

    pub fn new_f64_be_bytes<'anon_0>(v : &'anon_0 [byte]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_f64_be_bytes(v) };
        ret
    }

    pub fn new_from_owned(v : Box<[f64]>) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_from_owned(v) };
        ret
    }

    pub fn as_slice<'a>(&self) -> &'a [f64] {
        let ret = unsafe { Float64Vec_as_slice(self) };
        ret
    }

    pub fn fill_slice<'anon_0, 'anon_1>(&self, v : &'anon_1 mut [f64]) {
        let ret = unsafe { Float64Vec_fill_slice(self, v) };
        ret
    }

    pub fn set_value<'anon_0, 'anon_1>(&mut self, new_slice : &'anon_1 [f64]) {
        let ret = unsafe { Float64Vec_set_value(self, new_slice) };
        ret
    }

    pub fn to_string<'anon_0>(&self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { Float64Vec_to_string(self, write_mut) };
        let out_str = write.to_string();
        out_str
    }

    pub fn borrow<'a>(&self) -> &'a [f64] {
        let ret = unsafe { Float64Vec_borrow(self) };
        ret
    }

    pub fn get<'anon_0>(&self, i : usize) -> Option<f64> {
        let ret = unsafe { Float64Vec_get(self, i) };
        ret.into_converted_option()
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Float64Vec_new_bool<'anon_0>(v : &'anon_0 [bool]) -> Box<Float64Vec>;

    fn Float64Vec_new_i16<'anon_0>(v : &'anon_0 [i16]) -> Box<Float64Vec>;

    fn Float64Vec_new_u16<'anon_0>(v : &'anon_0 [u16]) -> Box<Float64Vec>;

    fn Float64Vec_new_isize<'anon_0>(v : &'anon_0 [isize]) -> Box<Float64Vec>;

    fn Float64Vec_new_usize<'anon_0>(v : &'anon_0 [usize]) -> Box<Float64Vec>;

    fn Float64Vec_new_f64_be_bytes<'anon_0>(v : &'anon_0 [byte]) -> Box<Float64Vec>;

    fn Float64Vec_new_from_owned(v : Box<[f64]>) -> Box<Float64Vec>;

    fn Float64Vec_as_slice<'a>(this: &Float64Vec) -> &'a [f64];

    fn Float64Vec_fill_slice<'anon_0, 'anon_1>(this: &Float64Vec, v : &'anon_1 mut [f64]);

    fn Float64Vec_set_value<'anon_0, 'anon_1>(this: &mut Float64Vec, new_slice : &'anon_1 [f64]);

    fn Float64Vec_to_string<'anon_0>(this: &Float64Vec, write_mut : &mut crate::DiplomatWrite) -> ();

    fn Float64Vec_borrow<'a>(this: &Float64Vec) -> &'a [f64];

    fn Float64Vec_get<'anon_0>(this: &Float64Vec, i : usize) -> diplomat_runtime::DiplomatOption<f64>;

    fn Float64Vec_destroy(this : *mut Float64Vec);
}