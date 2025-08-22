#[repr(C)]
pub struct RenamedStructWithAttrs {
    pub a: bool,
    pub b: u32,
}

impl RenamedStructWithAttrs {
    pub fn new_fallible(a : bool, b : u32) -> Result<RenamedStructWithAttrs, ()> {
            // TODO: writeable conversions.
        unsafe { namespace_StructWithAttrs_new_fallible(a, b) }
    }

    pub fn c(self) -> u32 {
            // TODO: writeable conversions.
        unsafe { namespace_StructWithAttrs_c(self) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_StructWithAttrs_new_fallible(a : bool, b : u32) -> DiplomatResult<RenamedStructWithAttrs, ()>;

    fn namespace_StructWithAttrs_c(this : RenamedStructWithAttrs) -> u32;

}