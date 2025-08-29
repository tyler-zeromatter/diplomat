use super::ImportedStruct;
use super::MyStruct;
use super::imported_struct::ImportedStructAbi;
use super::my_struct::MyStructAbi;
pub struct Opaque;

impl Drop for Opaque {
    fn drop(&mut self) {
        unsafe { Opaque_destroy(self) }
    }
}

impl Opaque {
    pub fn new() -> Box<Opaque> {
        let ret = unsafe { Opaque_new() };
        
        ret
    
    }

    pub fn try_from_utf8<'anon_0>(input : &'anon_0 [u8]) -> Option<Box<Opaque>> {
        let ret = unsafe { Opaque_try_from_utf8(input.into()) };
        
        ret
    
    }

    pub fn from_str<'anon_0>(input : &'anon_0 str) -> Box<Opaque> {
        let ret = unsafe { Opaque_from_str(input.into()) };
        
        ret
    
    }

    pub fn get_debug_str<'anon_0>(&'anon_0 self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        unsafe { Opaque_get_debug_str(self, write_mut) };
        
        let out_str = write.to_string();
        out_str
    
    }

    pub fn assert_struct<'anon_0>(&'anon_0 self, s : MyStruct) {
        unsafe { Opaque_assert_struct(self, s.into()) };
        
    }

    pub fn returns_usize() -> usize {
        let ret = unsafe { Opaque_returns_usize() };
        
        ret
    
    }

    pub fn returns_imported() -> ImportedStruct {
        let ret = unsafe { Opaque_returns_imported() };
        
        ret.from_ffi()
    
    }

    pub fn cmp() -> std::cmp::Ordering {
        let ret = unsafe { Opaque_cmp() };
        
        ret
    
    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Opaque_new() -> Box<Opaque>;

    fn Opaque_try_from_utf8<'anon_0>(input : diplomat_runtime::DiplomatStrSlice<'anon_0>) -> Option<Box<Opaque>>;

    fn Opaque_from_str<'anon_0>(input : diplomat_runtime::DiplomatUtf8StrSlice<'anon_0>) -> Box<Opaque>;

    fn Opaque_get_debug_str<'anon_0>(this: &'anon_0 Opaque, write_mut : &mut crate::DiplomatWrite) -> ();

    fn Opaque_assert_struct<'anon_0>(this: &'anon_0 Opaque, s : MyStructAbi);

    fn Opaque_returns_usize() -> usize;

    fn Opaque_returns_imported() -> ImportedStructAbi;

    fn Opaque_cmp() -> std::cmp::Ordering;

    fn Opaque_destroy(this : *mut Opaque);
}