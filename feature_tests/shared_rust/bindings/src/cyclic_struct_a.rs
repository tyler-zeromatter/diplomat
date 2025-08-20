use super::CyclicStructB;
#[repr(C)]
pub struct CyclicStructA {

}

impl CyclicStructA {
    pub fn get_b() -> CyclicStructB {
            // TODO: writeable conversions.
        unsafe { CyclicStructA_get_b() }
    }

    pub fn cyclic_out(self) {
            // TODO: writeable conversions.
        unsafe { CyclicStructA_cyclic_out(self, output) }
    }

    pub fn double_cyclic_out(self, cyclic_struct_a : CyclicStructA) {
            // TODO: writeable conversions.
        unsafe { CyclicStructA_double_cyclic_out(self, cyclic_struct_a, output) }
    }

    pub fn getter_out(self) {
            // TODO: writeable conversions.
        unsafe { CyclicStructA_getter_out(self, output) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn CyclicStructA_get_b() -> CyclicStructB;

    fn CyclicStructA_cyclic_out(this : CyclicStructA, output : &mut DiplomatWrite);

    fn CyclicStructA_double_cyclic_out(this : CyclicStructA, cyclic_struct_a : CyclicStructA, output : &mut DiplomatWrite);

    fn CyclicStructA_getter_out(this : CyclicStructA, output : &mut DiplomatWrite);

}