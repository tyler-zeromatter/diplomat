#[repr(C)]
pub struct StructWithSlices {

}

impl StructWithSlices {
    fn return_last(self) {
        unsafe { StructWithSlices_return_last(self) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn StructWithSlices_return_last(self);

}