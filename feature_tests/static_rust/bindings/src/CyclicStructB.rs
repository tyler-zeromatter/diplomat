use super::CyclicStructA;
#[repr(C)]
pub struct CyclicStructB {

}

impl CyclicStructB {
    fn get_a() -> CyclicStructA {
        unsafe { CyclicStructB_get_a() }
    }

    fn get_a_option() -> Option<CyclicStructA> {
        unsafe { CyclicStructB_get_a_option() }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn CyclicStructB_get_a() -> CyclicStructA;

    fn CyclicStructB_get_a_option() -> Option<CyclicStructA>;

}