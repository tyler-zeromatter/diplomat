pub struct OptionString;

impl Drop for OptionString {
    fn drop(&mut self) {
        unsafe { OptionString_destroy(self) }
    }
}

impl OptionString {
    pub fn new<'a>(diplomat_str : &'a [u8]) -> Option<Box<OptionString>> {
        let ret = unsafe { OptionString_new(diplomat_str.into()) };
        
        ret
    }

    pub fn write<'a>(&'a self) -> Result<String, ()> {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { OptionString_write(self, write_mut) };
        
        let out_str = write.to_string();
        ret.to_result().map(|_| {
            out_str
        })
    }

    pub fn borrow<'a>(&'a self) -> Option<&'a [u8]> {
        let ret = unsafe { OptionString_borrow(self) };
        
        ret.into_converted_option().map(|ok : diplomat_runtime::DiplomatStrSlice| { ok.into() })
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OptionString_new<'a>(diplomat_str : diplomat_runtime::DiplomatStrSlice<'a>) -> Option<Box<OptionString>>;

    fn OptionString_write<'a>(this: &'a OptionString, write_mut : &mut crate::DiplomatWrite) -> crate::DiplomatResult<(), ()>;

    fn OptionString_borrow<'a>(this: &'a OptionString) -> diplomat_runtime::DiplomatOption<diplomat_runtime::DiplomatStrSlice<'a>>;

    fn OptionString_destroy(this : *mut OptionString);
}