#[repr(C)]
pub struct StructWithSlices {

}

impl StructWithSlices {
    fn return_last() {
        unsafe { StructWithSlices_return_last() }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn StructWithSlices_return_last();

}