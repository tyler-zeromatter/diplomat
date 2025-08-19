#[repr(C)]
pub struct CyclicStructC {

}

impl CyclicStructC {
    fn takes_nested_parameters() {}
    fn cyclic_out() {}
}