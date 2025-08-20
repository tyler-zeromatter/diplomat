#[repr(C)]
pub struct StructWithSlices {
    pub first: &[TODO],
    pub second: &[u16],
}

impl StructWithSlices {
    pub fn return_last(self) {
            // TODO: writeable conversions.
        unsafe { StructWithSlices_return_last(self, output) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn StructWithSlices_return_last(this : StructWithSlices, output : &mut DiplomatWrite);

}