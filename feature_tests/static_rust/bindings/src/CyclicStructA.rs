#[repr(C)]
pub struct CyclicStructA {

}

impl CyclicStructA {
    fn get_b() {
        unsafe { CyclicStructA_get_b() }
    }

    fn cyclic_out() {
        unsafe { CyclicStructA_cyclic_out() }
    }

    fn double_cyclic_out() {
        unsafe { CyclicStructA_double_cyclic_out() }
    }

    fn getter_out() {
        unsafe { CyclicStructA_getter_out() }
    }

}