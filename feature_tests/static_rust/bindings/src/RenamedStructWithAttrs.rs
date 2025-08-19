#[repr(C)]
pub struct RenamedStructWithAttrs {

}

impl RenamedStructWithAttrs {
    fn new_fallible(a : bool, b : u32) {
        unsafe { namespace_StructWithAttrs_new_fallible(a, b) }
    }

    fn c(self) {
        unsafe { namespace_StructWithAttrs_c(self) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn namespace_StructWithAttrs_new_fallible(a : bool, b : u32);

    fn namespace_StructWithAttrs_c(self);

}