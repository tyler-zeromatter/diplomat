use super::CyclicStructA;
#[repr(C)]
pub struct CyclicStructB {

}

impl CyclicStructB {
    fn get_a() -> CyclicStructA {
            // TODO: writeable conversions.
        unsafe { CyclicStructB_get_a() }
    }

    fn get_a_option() -> Option<CyclicStructA> {
            // TODO: writeable conversions.
        unsafe { CyclicStructB_get_a_option() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn CyclicStructB_get_a() -> CyclicStructA;

    fn CyclicStructB_get_a_option() -> Option<CyclicStructA>;

}