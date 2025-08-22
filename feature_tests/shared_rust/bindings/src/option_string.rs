pub struct OptionString;

impl Drop for OptionString {
    fn drop(&mut self) {
        unsafe { OptionString_destroy(self) }
    }
}

impl OptionString {
    pub fn new(diplomat_str : &[u8]) -> Option<Box<OptionString>> {
        let ret = unsafe { OptionString_new(diplomat_str.into()) };
        ret
    }

    pub fn write(&self) -> Result<String, ()> {
        let write = diplomat_runtime::diplomat_buffer_write_create(0);
        let ret = unsafe { OptionString_write(self, write.as_mut().unwrap()) };
        // TODO: Create a helper in `lib.rs`.
        let out_str = unsafe {
            let write_ref = write.as_ref().unwrap();
            let buf = diplomat_runtime::diplomat_buffer_write_get_bytes(write_ref);
            let len = diplomat_runtime::diplomat_buffer_write_len(write_ref);
    
            if !buf.is_null() {
                // String takes ownership of the buffer:
                String::from_raw_parts(buf, len, len)
            } else {
                panic!("Could not read buffer, growth failed.")
            }
        };
        
        // Drop the write object, since we no longer need it:
        unsafe {
            drop(Box::from_raw(write))
        }
        // TODO: Check DiplomatOption with this method.
        ret.map(|_| {
            out_str
        }).into()
    }

    pub fn borrow(&self) -> Option<&[u8]> {
        let ret = unsafe { OptionString_borrow(self) };
        ret.into_converted_option()
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OptionString_new(diplomat_str : diplomat_runtime::DiplomatStrSlice) -> Option<Box<OptionString>>;

    fn OptionString_write(this: &OptionString, write : &mut diplomat_runtime::DiplomatWrite) -> diplomat_runtime::DiplomatResult<(), ()>;

    fn OptionString_borrow(this: &OptionString) -> diplomat_runtime::DiplomatOption<diplomat_runtime::DiplomatStrSlice>;

    fn OptionString_destroy(this : *mut OptionString);
}