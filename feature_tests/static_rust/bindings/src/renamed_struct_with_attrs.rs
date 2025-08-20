#[repr(C)]
pub struct RenamedStructWithAttrs {

}

impl RenamedStructWithAttrs {
    fn new_fallible(a : bool, b : u32) -> Result<RenamedStructWithAttrs, ()> {
            // TODO: writeable conversions.
        unsafe { namespace_StructWithAttrs_new_fallible(a, b) }
    }

    fn c(mut self) -> u32 {
            // TODO: writeable conversions.
        unsafe { namespace_StructWithAttrs_c(self) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_StructWithAttrs_new_fallible(a : bool, b : u32) -> Result<RenamedStructWithAttrs, ()>;

    fn namespace_StructWithAttrs_c(mut this : RenamedStructWithAttrs) -> u32;

}