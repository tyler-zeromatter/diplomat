#[repr(C)]
pub struct CyclicStructB {

}

impl CyclicStructB {
    fn get_a() {
        unsafe { CyclicStructB_get_a() }
    }

    fn get_a_option() {
        unsafe { CyclicStructB_get_a_option() }
    }

}