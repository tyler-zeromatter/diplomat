pub struct Utf16Wrap;

impl Drop for Utf16Wrap {
    fn drop(&mut self) {
        unsafe { Utf16Wrap_destroy(self) }
    }
}

impl Utf16Wrap {
    pub fn from_utf16<'anon_0>(input : &'anon_0 [u16]) -> Box<Utf16Wrap> {
        let ret = unsafe { Utf16Wrap_from_utf16(input.into()) };
        
        ret
    
    }

    pub fn get_debug_str<'anon_0>(&'anon_0 self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        unsafe { Utf16Wrap_get_debug_str(self, write_mut) };
        
        let out_str = write.to_string();
        out_str
    
    }

    pub fn borrow_cont<'a>(&'a self) -> &'a [u16] {
        let ret = unsafe { Utf16Wrap_borrow_cont(self) };
        
        ret.into()
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Utf16Wrap_from_utf16<'anon_0>(input : diplomat_runtime::DiplomatStr16Slice<'anon_0>) -> Box<Utf16Wrap>;

    fn Utf16Wrap_get_debug_str<'anon_0>(this: &'anon_0 Utf16Wrap, write_mut : &mut crate::DiplomatWrite) -> ();

    fn Utf16Wrap_borrow_cont<'a>(this: &'a Utf16Wrap) -> diplomat_runtime::DiplomatStr16Slice<'a>;

    fn Utf16Wrap_destroy(this : *mut Utf16Wrap);
}