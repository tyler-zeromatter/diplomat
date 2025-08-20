#[repr(C)]
pub struct StructWithSlices {

}

impl StructWithSlices {
    fn return_last(self) {
            // TODO: writeable conversions.
        unsafe { StructWithSlices_return_last(self, output) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn StructWithSlices_return_last(this : StructWithSlices, output : &mut DiplomatWrite);

}