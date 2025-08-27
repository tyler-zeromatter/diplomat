use super::ImportedStruct;
use super::MyStruct;
use std::marker::PhantomData;

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

    pub fn from_str<'anon_0>(input : &'anon_0 String) -> Box<Opaque> {
        let ret = unsafe { Opaque_from_str(input.into()) };
        ret
    }

    pub fn get_debug_str<'anon_0>(&self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { Opaque_get_debug_str(self, write_mut) };
        let out_str = write.to_string();
        out_str
    }

    pub fn assert_struct<'anon_0>(&self, s : MyStruct) {
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
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Opaque_new() -> Box<Opaque>;

    fn Opaque_try_from_utf8<'anon_0>(input : &'anon_0 diplomat_runtime::DiplomatStrSlice) -> Option<Box<Opaque>>;

    fn Opaque_from_str<'anon_0>(input : &'anon_0 diplomat_runtime::DiplomatStrSlice) -> Box<Opaque>;

    fn Opaque_get_debug_str<'anon_0>(this: &Opaque, write_mut : &mut crate::DiplomatWrite) -> ();

    fn Opaque_assert_struct<'anon_0>(this: &Opaque, s : MyStruct);

    fn Opaque_returns_usize() -> usize;

    fn Opaque_returns_imported() -> ImportedStruct;

    fn Opaque_cmp() -> ordering;

    fn Opaque_destroy(this : *mut Opaque);
}