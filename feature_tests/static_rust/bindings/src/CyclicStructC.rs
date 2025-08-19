#[repr(C)]
pub struct CyclicStructC {

}

impl CyclicStructC {
    fn takes_nested_parameters() {
        unsafe { CyclicStructC_takes_nested_parameters() }
    }

    fn cyclic_out() {
        unsafe { CyclicStructC_cyclic_out() }
    }

}