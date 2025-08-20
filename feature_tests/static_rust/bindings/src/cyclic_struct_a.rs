use super::CyclicStructB;
#[repr(C)]
pub struct CyclicStructA {

}

impl CyclicStructA {
    fn get_b() -> CyclicStructB {
            // TODO: writeable conversions.
        unsafe { CyclicStructA_get_b() }
    }

    fn cyclic_out(mut self) {
            // TODO: writeable conversions.
        unsafe { CyclicStructA_cyclic_out(self, output) }
    }

    fn double_cyclic_out(mut self, cyclic_struct_a : CyclicStructA) {
            // TODO: writeable conversions.
        unsafe { CyclicStructA_double_cyclic_out(self, cyclic_struct_a, output) }
    }

    fn getter_out(mut self) {
            // TODO: writeable conversions.
        unsafe { CyclicStructA_getter_out(self, output) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn CyclicStructA_get_b() -> CyclicStructB;

    fn CyclicStructA_cyclic_out(mut this : CyclicStructA, output : &mut DiplomatWrite);

    fn CyclicStructA_double_cyclic_out(mut this : CyclicStructA, cyclic_struct_a : CyclicStructA, output : &mut DiplomatWrite);

    fn CyclicStructA_getter_out(mut this : CyclicStructA, output : &mut DiplomatWrite);

}