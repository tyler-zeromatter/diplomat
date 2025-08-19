#[repr(C)]
pub struct CyclicStructA {

}

impl CyclicStructA {
    fn get_b() {
        unsafe { CyclicStructA_get_b() }
    }

    fn cyclic_out(self) {
        unsafe { CyclicStructA_cyclic_out(self) }
    }

    fn double_cyclic_out(self, cyclic_struct_a : CyclicStructA) {
        unsafe { CyclicStructA_double_cyclic_out(self, cyclic_struct_a) }
    }

    fn getter_out(self) {
        unsafe { CyclicStructA_getter_out(self) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn CyclicStructA_get_b();

    fn CyclicStructA_cyclic_out(self);

    fn CyclicStructA_double_cyclic_out(self, cyclic_struct_a : CyclicStructA);

    fn CyclicStructA_getter_out(self);

}