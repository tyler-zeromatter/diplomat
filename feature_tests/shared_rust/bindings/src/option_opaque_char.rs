pub struct OptionOpaqueChar;

impl Drop for OptionOpaqueChar {
    fn drop(&mut self) {
        unsafe { OptionOpaqueChar_destroy(self) }
    }
}

impl OptionOpaqueChar {
    pub fn assert_char<'anon_0>(&'anon_0 self, ch : diplomat_runtime::DiplomatChar) {
        let ret = unsafe { OptionOpaqueChar_assert_char(self, ch) };
        }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OptionOpaqueChar_assert_char<'anon_0>(this: &'anon_0 OptionOpaqueChar, ch : diplomat_runtime::DiplomatChar);

    fn OptionOpaqueChar_destroy(this : *mut OptionOpaqueChar);
}