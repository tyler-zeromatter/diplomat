use super::CyclicStructA;
#[repr(C)]
pub struct CyclicStructC {
    pub a: CyclicStructA,
}

impl CyclicStructC {
    pub fn takes_nested_parameters(c : CyclicStructC) -> CyclicStructC {
            // TODO: writeable conversions.
        unsafe { CyclicStructC_takes_nested_parameters(c) }
    }

    pub fn cyclic_out(self) {
            // TODO: writeable conversions.
        unsafe { CyclicStructC_cyclic_out(self, output) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn CyclicStructC_takes_nested_parameters(c : CyclicStructC) -> CyclicStructC;

    fn CyclicStructC_cyclic_out(this : CyclicStructC, output : &mut DiplomatWrite);

}