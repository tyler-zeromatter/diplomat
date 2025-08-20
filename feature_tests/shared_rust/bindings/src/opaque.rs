use super::ImportedStruct;
use super::MyStruct;
pub struct Opaque;

impl Opaque {
    pub fn new() -> Box<Opaque> {
            // TODO: writeable conversions.
        unsafe { Opaque_new() }
    }

    pub fn try_from_utf8(input : &[TODO]) -> Box<Option<Opaque>> {
            // TODO: writeable conversions.
        unsafe { Opaque_try_from_utf8(input) }
    }

    pub fn from_str(input : &[TODO]) -> Box<Opaque> {
            // TODO: writeable conversions.
        unsafe { Opaque_from_str(input) }
    }

    pub fn get_debug_str(&self) {
            // TODO: writeable conversions.
        unsafe { Opaque_get_debug_str(self, output) }
    }

    pub fn assert_struct(&self, s : MyStruct) {
            // TODO: writeable conversions.
        unsafe { Opaque_assert_struct(self, s) }
    }

    pub fn returns_usize() -> usize {
            // TODO: writeable conversions.
        unsafe { Opaque_returns_usize() }
    }

    pub fn returns_imported() -> ImportedStruct {
            // TODO: writeable conversions.
        unsafe { Opaque_returns_imported() }
    }

    pub fn cmp() -> ordering {
            // TODO: writeable conversions.
        unsafe { Opaque_cmp() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Opaque_new() -> Box<Opaque>;

    fn Opaque_try_from_utf8(input : &[TODO]) -> Box<Option<Opaque>>;

    fn Opaque_from_str(input : &[TODO]) -> Box<Opaque>;

    fn Opaque_get_debug_str(this: &Opaque, output : &mut DiplomatWrite);

    fn Opaque_assert_struct(this: &Opaque, s : MyStruct);

    fn Opaque_returns_usize() -> usize;

    fn Opaque_returns_imported() -> ImportedStruct;

    fn Opaque_cmp() -> ordering;

}