pub struct OptionString;

impl OptionString {
    pub fn new(diplomat_str : &[u8]) -> Box<Option<OptionString>> {
        let ret = unsafe { OptionString_new(diplomat_str) };
        ret
    }

    pub fn write(&self) -> Result<String, ()> {
        let write = unsafe {
            diplomat_runtime::diplomat_buffer_write_create(0)
        };
        let ret = unsafe { OptionString_write(self, write.as_mut().unwrap()) };
        let out_str = unsafe {
            let write_ref = write.as_ref().unwrap();
            let buf = diplomat_runtime::diplomat_buffer_write_get_bytes(write_ref);
            let len = diplomat_runtime::diplomat_buffer_write_len(write_ref);
    
            if !buf.is_null() {
                String::from_raw_parts(buf, len, len)
            } else {
                panic!("Could not read buffer, growth failed.")
            }
        };
    
        unsafe {
            diplomat_runtime::diplomat_buffer_write_destroy(write);
        }
        out_str
    }

    pub fn borrow(&self) -> Option<&[u8]> {
        let ret = unsafe { OptionString_borrow(self) };
        ret.into()
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OptionString_new(diplomat_str : diplomat_runtime::DiplomatStrSlice) -> Box<Option<OptionString>>;

    fn OptionString_write(this: &OptionString, write : &mut diplomat_runtime::DiplomatWrite) -> diplomat_runtime::DiplomatResult<String, ()>;

    fn OptionString_borrow(this: &OptionString) -> diplomat_runtime::DiplomatOption<diplomat_runtime::DiplomatStrSlice>;

}