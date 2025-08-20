pub struct Float64Vec;

impl Float64Vec {
    fn new_bool(v : TODO()) -> Float64Vec {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_bool(v) }
    }

    fn new_i16(v : TODO()) -> Float64Vec {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_i16(v) }
    }

    fn new_u16(v : TODO()) -> Float64Vec {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_u16(v) }
    }

    fn new_isize(v : TODO()) -> Float64Vec {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_isize(v) }
    }

    fn new_usize(v : TODO()) -> Float64Vec {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_usize(v) }
    }

    fn new_f64_be_bytes(v : TODO()) -> Float64Vec {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_f64_be_bytes(v) }
    }

    fn new_from_owned(v : TODO()) -> Float64Vec {
            // TODO: writeable conversions.
        unsafe { Float64Vec_new_from_owned(v) }
    }

    fn as_slice(&self) -> TODO() {
            // TODO: writeable conversions.
        unsafe { Float64Vec_as_slice(self) }
    }

    fn fill_slice(&self, v : TODO()) {
            // TODO: writeable conversions.
        unsafe { Float64Vec_fill_slice(self, v) }
    }

    fn set_value(&mut self, new_slice : TODO()) {
            // TODO: writeable conversions.
        unsafe { Float64Vec_set_value(self, new_slice) }
    }

    fn to_string(&self) {
            // TODO: writeable conversions.
        unsafe { Float64Vec_to_string(self, output) }
    }

    fn borrow(&self) -> TODO() {
            // TODO: writeable conversions.
        unsafe { Float64Vec_borrow(self) }
    }

    fn get(&self, i : usize) -> Option<f64> {
            // TODO: writeable conversions.
        unsafe { Float64Vec_get(self, i) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Float64Vec_new_bool(v : TODO()) -> Float64Vec;

    fn Float64Vec_new_i16(v : TODO()) -> Float64Vec;

    fn Float64Vec_new_u16(v : TODO()) -> Float64Vec;

    fn Float64Vec_new_isize(v : TODO()) -> Float64Vec;

    fn Float64Vec_new_usize(v : TODO()) -> Float64Vec;

    fn Float64Vec_new_f64_be_bytes(v : TODO()) -> Float64Vec;

    fn Float64Vec_new_from_owned(v : TODO()) -> Float64Vec;

    fn Float64Vec_as_slice(this: &Float64Vec) -> TODO();

    fn Float64Vec_fill_slice(this: &Float64Vec, v : TODO());

    fn Float64Vec_set_value(this: &mut Float64Vec, new_slice : TODO());

    fn Float64Vec_to_string(this: &Float64Vec, output : &mut DiplomatWrite);

    fn Float64Vec_borrow(this: &Float64Vec) -> TODO();

    fn Float64Vec_get(this: &Float64Vec, i : usize) -> Option<f64>;

}