pub struct Utf16Wrap;

impl Utf16Wrap {
    fn from_utf16(input : &[TODO]) -> Utf16Wrap {
            // TODO: writeable conversions.
        unsafe { Utf16Wrap_from_utf16(input) }
    }

    fn get_debug_str(&self) {
            // TODO: writeable conversions.
        unsafe { Utf16Wrap_get_debug_str(self, output) }
    }

    fn borrow_cont(&self) -> &[TODO] {
            // TODO: writeable conversions.
        unsafe { Utf16Wrap_borrow_cont(self) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Utf16Wrap_from_utf16(input : &[TODO]) -> Utf16Wrap;

    fn Utf16Wrap_get_debug_str(this: &Utf16Wrap, output : &mut DiplomatWrite);

    fn Utf16Wrap_borrow_cont(this: &Utf16Wrap) -> &[TODO];

}