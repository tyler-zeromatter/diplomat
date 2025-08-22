pub struct Float64Vec;

impl Drop for Float64Vec {
    fn drop(&mut self) {
        unsafe { Float64Vec_destroy(self) }
    }
}

impl Float64Vec {
    pub fn new_bool(v : &[bool]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_bool(v) };
        ret
    }

    pub fn new_i16(v : &[i16]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_i16(v) };
        ret
    }

    pub fn new_u16(v : &[u16]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_u16(v) };
        ret
    }

    pub fn new_isize(v : &[isize]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_isize(v) };
        ret
    }

    pub fn new_usize(v : &[usize]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_usize(v) };
        ret
    }

    pub fn new_f64_be_bytes(v : &[byte]) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_f64_be_bytes(v) };
        ret
    }

    pub fn new_from_owned(v : Box<[f64]>) -> Box<Float64Vec> {
        let ret = unsafe { Float64Vec_new_from_owned(v) };
        ret
    }

    pub fn as_slice(&self) -> &[f64] {
        let ret = unsafe { Float64Vec_as_slice(self) };
        ret
    }

    pub fn fill_slice(&self, v : &mut [f64]) {
        let ret = unsafe { Float64Vec_fill_slice(self, v) };
        ret
    }

    pub fn set_value(&mut self, new_slice : &[f64]) {
        let ret = unsafe { Float64Vec_set_value(self, new_slice) };
        ret
    }

    pub fn to_string(&self) -> String {
        let write = diplomat_runtime::diplomat_buffer_write_create(0);
        let ret = unsafe { Float64Vec_to_string(self, write.as_mut().unwrap()) };
        // TODO: Create a helper in `lib.rs`.
        let out_str = unsafe {
            let write_ref = write.as_ref().unwrap();
            let buf = diplomat_runtime::diplomat_buffer_write_get_bytes(write_ref);
            let len = diplomat_runtime::diplomat_buffer_write_len(write_ref);
    
            if !buf.is_null() {
                // String takes ownership of the buffer:
                String::from_raw_parts(buf, len, len)
            } else {
                panic!("Could not read buffer, growth failed.")
            }
        };
        out_str
    }

    pub fn borrow(&self) -> &[f64] {
        let ret = unsafe { Float64Vec_borrow(self) };
        ret
    }

    pub fn get(&self, i : usize) -> Option<f64> {
        let ret = unsafe { Float64Vec_get(self, i) };
        ret.into_converted_option()
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

    fn Float64Vec_to_string(this: &Float64Vec, write : &mut diplomat_runtime::DiplomatWrite) -> ();

    fn Float64Vec_borrow(this: &Float64Vec) -> &[f64];

    fn Float64Vec_get(this: &Float64Vec, i : usize) -> diplomat_runtime::DiplomatOption<f64>;

    fn Float64Vec_destroy(this : *mut Float64Vec);

}