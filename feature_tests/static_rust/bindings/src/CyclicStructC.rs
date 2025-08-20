#[repr(C)]
pub struct CyclicStructC {

}

impl CyclicStructC {
    fn takes_nested_parameters(c : CyclicStructC) -> CyclicStructC {
        unsafe { CyclicStructC_takes_nested_parameters(c) }
    }

    fn cyclic_out(self) {
        unsafe { CyclicStructC_cyclic_out(self, output) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn CyclicStructC_takes_nested_parameters(c : CyclicStructC) -> CyclicStructC;

    fn CyclicStructC_cyclic_out(self, output : &mut DiplomatWrite);

}