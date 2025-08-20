pub struct OptionOpaqueChar;

impl OptionOpaqueChar {
    pub fn assert_char(&self, ch : diplomat_runtime::DiplomatChar) {
            // TODO: writeable conversions.
        unsafe { OptionOpaqueChar_assert_char(self, ch) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OptionOpaqueChar_assert_char(this: &OptionOpaqueChar, ch : diplomat_runtime::DiplomatChar);

}