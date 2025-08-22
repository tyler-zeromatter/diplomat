#[repr(C)]
pub struct RenamedStructWithAttrs {
    pub a: bool,
    pub b: u32,
}

impl RenamedStructWithAttrs {
    pub fn new_fallible(a : bool, b : u32) -> Result<RenamedStructWithAttrs, ()> {
        let ret = unsafe { namespace_StructWithAttrs_new_fallible(a, b) };
        ret.into()
    }

    pub fn c(self) -> u32 {
        let ret = unsafe { namespace_StructWithAttrs_c(self) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_StructWithAttrs_new_fallible(a : bool, b : u32) -> diplomat_runtime::DiplomatResult<RenamedStructWithAttrs, ()>;

    fn namespace_StructWithAttrs_c(this : RenamedStructWithAttrs) -> u32;
}