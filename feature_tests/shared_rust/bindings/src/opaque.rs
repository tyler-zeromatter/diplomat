use super::ImportedStruct;
use super::MyStruct;
pub struct Opaque;

impl Opaque {
    pub fn new() -> Box<Opaque> {
        let ret = unsafe { Opaque_new() };
        ret
    }

    pub fn try_from_utf8(input : &[u8]) -> Option<Box<Opaque>> {
        let ret = unsafe { Opaque_try_from_utf8(input.into()) };
        ret
    }

    pub fn from_str(input : &String) -> Box<Opaque> {
        let ret = unsafe { Opaque_from_str(input.into()) };
        ret
    }

    pub fn get_debug_str(&self) -> String {
        let write = diplomat_runtime::diplomat_buffer_write_create(0);
        let ret = unsafe { Opaque_get_debug_str(self, write.as_mut().unwrap()) };
        // TODO: Create a helper in `lib.rs`.
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

    pub fn assert_struct(&self, s : MyStruct) {
        let ret = unsafe { Opaque_assert_struct(self, s) };
        ret
    }

    pub fn returns_usize() -> usize {
        let ret = unsafe { Opaque_returns_usize() };
        ret
    }

    pub fn returns_imported() -> ImportedStruct {
        let ret = unsafe { Opaque_returns_imported() };
        ret
    }

    pub fn cmp() -> ordering {
        let ret = unsafe { Opaque_cmp() };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Opaque_new() -> Box<Opaque>;

    fn Opaque_try_from_utf8(input : diplomat_runtime::DiplomatStrSlice) -> Option<Box<Opaque>>;

    fn Opaque_from_str(input : diplomat_runtime::DiplomatStrSlice) -> Box<Opaque>;

    fn Opaque_get_debug_str(this: &Opaque, write : &mut diplomat_runtime::DiplomatWrite) -> ();

    fn Opaque_assert_struct(this: &Opaque, s : MyStruct);

    fn Opaque_returns_usize() -> usize;

    fn Opaque_returns_imported() -> ImportedStruct;

    fn Opaque_cmp() -> ordering;

}