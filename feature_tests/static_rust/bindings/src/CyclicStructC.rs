#[repr(C)]
pub struct CyclicStructC {

}

impl CyclicStructC {
    fn takes_nested_parameters(c : CyclicStructC) {
        unsafe { CyclicStructC_takes_nested_parameters(c) }
    }

    fn cyclic_out(self) {
        unsafe { CyclicStructC_cyclic_out(self) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn CyclicStructC_takes_nested_parameters(c : CyclicStructC);

    fn CyclicStructC_cyclic_out(self);

}