pub struct OptionString;

impl Drop for OptionString {
    fn drop(&mut self) {
        unsafe { OptionString_destroy(self) }
    }
}

impl OptionString {
    pub fn new(diplomat_str : [u8]) -> Option<Box<OptionString>> {
        let ret = unsafe { OptionString_new(diplomat_str.into()) };
        ret
    }

    pub fn write(&self) -> Result<String, ()> {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { OptionString_write(self, write_mut) };
        let out_str = write.to_string();
        if ret.is_ok {
            Ok(out_str)
        } else {
            Err(ret.unwrap_err())
        }
    }

    pub fn borrow(&self) -> Option<[u8]> {
        let ret = unsafe { OptionString_borrow(self) };
        ret.into_converted_option()
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OptionString_new(diplomat_str : diplomat_runtime::DiplomatStrSlice) -> Option<Box<OptionString>>;

    fn OptionString_write(this: &OptionString, write_mut : &mut crate::DiplomatWrite) -> crate::DiplomatResult<(), ()>;

    fn OptionString_borrow(this: &OptionString) -> diplomat_runtime::DiplomatOption<diplomat_runtime::DiplomatStrSlice>;

    fn OptionString_destroy(this : *mut OptionString);
}