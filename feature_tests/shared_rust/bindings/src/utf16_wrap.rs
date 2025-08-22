pub struct Utf16Wrap;

impl Utf16Wrap {
    pub fn from_utf16(input : &[u16]) -> Box<Utf16Wrap> {
        let ret = unsafe { Utf16Wrap_from_utf16(input) };
        ret
    }

    pub fn get_debug_str(&self) -> String {
        let write = unsafe {
            diplomat_runtime::diplomat_buffer_write_create(0)
        };
        let ret = unsafe { Utf16Wrap_get_debug_str(self, write.as_mut().unwrap()) };
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

    pub fn borrow_cont(&self) -> &[u16] {
        let ret = unsafe { Utf16Wrap_borrow_cont(self) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Utf16Wrap_from_utf16(input : diplomat_runtime::DiplomatStrSlice) -> Box<Utf16Wrap>;

    fn Utf16Wrap_get_debug_str(this: &Utf16Wrap, write : &mut diplomat_runtime::DiplomatWrite) -> String;

    fn Utf16Wrap_borrow_cont(this: &Utf16Wrap) -> diplomat_runtime::DiplomatStrSlice;

}