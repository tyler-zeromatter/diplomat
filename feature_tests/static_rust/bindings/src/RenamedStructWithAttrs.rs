#[repr(C)]
pub struct RenamedStructWithAttrs {

}

impl RenamedStructWithAttrs {
    fn new_fallible() {
        unsafe { namespace_StructWithAttrs_new_fallible() }
    }

    fn c() {
        unsafe { namespace_StructWithAttrs_c() }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn namespace_StructWithAttrs_new_fallible();

    fn namespace_StructWithAttrs_c();

}