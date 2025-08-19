#[repr(C)]
pub struct CyclicStructA {

}

impl CyclicStructA {
    fn get_b() {}
    fn cyclic_out() {}
    fn double_cyclic_out() {}
    fn getter_out() {}
}