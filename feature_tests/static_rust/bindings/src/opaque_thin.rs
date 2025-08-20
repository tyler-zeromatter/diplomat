pub struct OpaqueThin;

impl OpaqueThin {
    fn a(&self) -> i32 {
            // TODO: writeable conversions.
        unsafe { OpaqueThin_a(self) }
    }

    fn b(&self) -> f32 {
            // TODO: writeable conversions.
        unsafe { OpaqueThin_b(self) }
    }

    fn c(&self) {
            // TODO: writeable conversions.
        unsafe { OpaqueThin_c(self, output) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OpaqueThin_a(this: &OpaqueThin) -> i32;

    fn OpaqueThin_b(this: &OpaqueThin) -> f32;

    fn OpaqueThin_c(this: &OpaqueThin, output : &mut DiplomatWrite);

}