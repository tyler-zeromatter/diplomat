pub struct OptionOpaqueChar;

impl Drop for OptionOpaqueChar {
    fn drop(&mut self) {
        unsafe { OptionOpaqueChar_destroy(self) }
    }
}

impl OptionOpaqueChar {
    pub fn assert_char(&self, ch : diplomat_runtime::DiplomatChar) {
        let ret = unsafe { OptionOpaqueChar_assert_char(self, ch) };
        ret
    }

    

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OptionOpaqueChar_assert_char(this: &OptionOpaqueChar, ch : diplomat_runtime::DiplomatChar);

    fn OptionOpaqueChar_destroy(this : *mut OptionOpaqueChar);

}