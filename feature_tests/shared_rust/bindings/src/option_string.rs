pub struct OptionString;

impl Drop for OptionString {
    fn drop(&mut self) {
        unsafe { OptionString_destroy(self) }
    }
}

impl OptionString {
    pub fn new<'a>(diplomat_str : &'a [u8]) -> Box<Option<OptionString>> {
        let ret = unsafe { OptionString_new(diplomat_str.into()) };
        ret
    }

    pub fn write<'a>(&'a self) -> Result<String, ()> {
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

    pub fn borrow<'a>(&'a self) -> Option<[u8]> {
        let ret = unsafe { OptionString_borrow(self) };
        ret.into_converted_option()
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OptionString_new<'a>(diplomat_str : &'a diplomat_runtime::DiplomatStrSlice) -> Box<Option<OptionString>>;

    fn OptionString_write<'a>(this: &'a OptionString, write_mut : &mut crate::DiplomatWrite) -> crate::DiplomatResult<(), ()>;

    fn OptionString_borrow<'a>(this: &'a OptionString) -> diplomat_runtime::DiplomatOption<diplomat_runtime::DiplomatStrSlice>;

    fn OptionString_destroy(this : *mut OptionString);
}