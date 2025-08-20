#[repr(C)]
pub struct CyclicStructC {

}

impl CyclicStructC {
    fn takes_nested_parameters(c : CyclicStructC) -> CyclicStructC {
            // TODO: writeable conversions.
        unsafe { CyclicStructC_takes_nested_parameters(c) }
    }

    fn cyclic_out(mut self) {
            // TODO: writeable conversions.
        unsafe { CyclicStructC_cyclic_out(self, output) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn CyclicStructC_takes_nested_parameters(c : CyclicStructC) -> CyclicStructC;

    fn CyclicStructC_cyclic_out(mut this : CyclicStructC, output : &mut DiplomatWrite);

}