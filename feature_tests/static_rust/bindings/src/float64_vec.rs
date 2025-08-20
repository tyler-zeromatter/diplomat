pub struct Float64Vec;

impl Float64Vec {
    pub fn new_bool(v : &[bool]) -> Box<Float64Vec> {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_bool(v) }
    }

    pub fn new_i16(v : &[i16]) -> Box<Float64Vec> {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_i16(v) }
    }

    pub fn new_u16(v : &[u16]) -> Box<Float64Vec> {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_u16(v) }
    }

    pub fn new_isize(v : &[isize]) -> Box<Float64Vec> {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_isize(v) }
    }

    pub fn new_usize(v : &[usize]) -> Box<Float64Vec> {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_usize(v) }
    }

    pub fn new_f64_be_bytes(v : &[byte]) -> Box<Float64Vec> {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_f64_be_bytes(v) }
    }

    pub fn new_from_owned(v : Box<[f64]>) -> Box<Float64Vec> {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_from_owned(v) }
    }

    pub fn as_slice(&self) -> &[f64] {
            // TODO: writeable conversions.
        unsafe { Float64Vec_as_slice(self) }
    }

    pub fn fill_slice(&self, v : &mut [f64]) {
            // TODO: writeable conversions.
        unsafe { Float64Vec_fill_slice(self, v) }
    }

    pub fn set_value(&mut self, new_slice : &[f64]) {
            // TODO: writeable conversions.
        unsafe { Float64Vec_set_value(self, new_slice) }
    }

    pub fn to_string(&self) {
            // TODO: writeable conversions.
        unsafe { Float64Vec_to_string(self, output) }
    }

    pub fn borrow(&self) -> &[f64] {
            // TODO: writeable conversions.
        unsafe { Float64Vec_borrow(self) }
    }

    pub fn get(&self, i : usize) -> Option<f64> {
            // TODO: writeable conversions.
        unsafe { Float64Vec_get(self, i) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Float64Vec_new_bool(v : &[bool]) -> Box<Float64Vec>;

    fn Float64Vec_new_i16(v : &[i16]) -> Box<Float64Vec>;

    fn Float64Vec_new_u16(v : &[u16]) -> Box<Float64Vec>;

    fn Float64Vec_new_isize(v : &[isize]) -> Box<Float64Vec>;

    fn Float64Vec_new_usize(v : &[usize]) -> Box<Float64Vec>;

    fn Float64Vec_new_f64_be_bytes(v : &[byte]) -> Box<Float64Vec>;

    fn Float64Vec_new_from_owned(v : Box<[f64]>) -> Box<Float64Vec>;

    fn Float64Vec_as_slice(this: &Float64Vec) -> &[f64];

    fn Float64Vec_fill_slice(this: &Float64Vec, v : &mut [f64]);

    fn Float64Vec_set_value(this: &mut Float64Vec, new_slice : &[f64]);

    fn Float64Vec_to_string(this: &Float64Vec, output : &mut DiplomatWrite);

    fn Float64Vec_borrow(this: &Float64Vec) -> &[f64];

    fn Float64Vec_get(this: &Float64Vec, i : usize) -> Option<f64>;

}