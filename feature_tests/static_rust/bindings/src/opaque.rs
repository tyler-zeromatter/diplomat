use super::ImportedStruct;
use super::MyStruct;
pub struct Opaque;

impl Opaque {
    fn new() -> Opaque {
            // TODO: writeable conversions.
        unsafe { Opaque_new() }
    }

    fn try_from_utf8(input : TODO()) -> Opaque {
            // TODO: writeable conversions.
        unsafe { Opaque_try_from_utf8(input) }
    }

    fn from_str(input : TODO()) -> Opaque {
            // TODO: writeable conversions.
        unsafe { Opaque_from_str(input) }
    }

    fn get_debug_str(&self) {
            // TODO: writeable conversions.
        unsafe { Opaque_get_debug_str(self, output) }
    }

    fn assert_struct(&self, s : MyStruct) {
            // TODO: writeable conversions.
        unsafe { Opaque_assert_struct(self, s) }
    }

    fn returns_usize() -> usize {
            // TODO: writeable conversions.
        unsafe { Opaque_returns_usize() }
    }

    fn returns_imported() -> ImportedStruct {
            // TODO: writeable conversions.
        unsafe { Opaque_returns_imported() }
    }

    fn cmp() -> ordering {
            // TODO: writeable conversions.
        unsafe { Opaque_cmp() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Opaque_new() -> Opaque;

    fn Opaque_try_from_utf8(input : TODO()) -> Opaque;

    fn Opaque_from_str(input : TODO()) -> Opaque;

    fn Opaque_get_debug_str(this: &Opaque, output : &mut DiplomatWrite);

    fn Opaque_assert_struct(this: &Opaque, s : MyStruct);

    fn Opaque_returns_usize() -> usize;

    fn Opaque_returns_imported() -> ImportedStruct;

    fn Opaque_cmp() -> ordering;

}