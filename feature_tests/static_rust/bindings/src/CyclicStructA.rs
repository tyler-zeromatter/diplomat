use super::CyclicStructB;
#[repr(C)]
pub struct CyclicStructA {

}

impl CyclicStructA {
    fn get_b() -> CyclicStructB {
        unsafe { CyclicStructA_get_b() }
    }

    fn cyclic_out(self) {
        unsafe { CyclicStructA_cyclic_out(self, output) }
    }

    fn double_cyclic_out(self, cyclic_struct_a : CyclicStructA) {
        unsafe { CyclicStructA_double_cyclic_out(self, cyclic_struct_a, output) }
    }

    fn getter_out(self) {
        unsafe { CyclicStructA_getter_out(self, output) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn CyclicStructA_get_b() -> CyclicStructB;

    fn CyclicStructA_cyclic_out(self, output : &mut DiplomatWrite);

    fn CyclicStructA_double_cyclic_out(self, cyclic_struct_a : CyclicStructA, output : &mut DiplomatWrite);

    fn CyclicStructA_getter_out(self, output : &mut DiplomatWrite);

}