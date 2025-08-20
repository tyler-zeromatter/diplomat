pub struct OptionOpaqueChar;

impl OptionOpaqueChar {
    fn assert_char(&self, ch : char) {
            // TODO: writeable conversions.
        unsafe { OptionOpaqueChar_assert_char(self, ch) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OptionOpaqueChar_assert_char(this: &OptionOpaqueChar, ch : char);

}