use super::CyclicStructA;
#[repr(C)]
pub struct CyclicStructB {
    pub field: u8,
}

impl CyclicStructB {
    pub fn get_a() -> CyclicStructA {
        let ret = unsafe { CyclicStructB_get_a() };
        ret
    }

    pub fn get_a_option() -> Option<CyclicStructA> {
        let ret = unsafe { CyclicStructB_get_a_option() };
        ret.into_converted_option()
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn CyclicStructB_get_a() -> CyclicStructA;

    fn CyclicStructB_get_a_option() -> diplomat_runtime::DiplomatOption<CyclicStructA>;
}