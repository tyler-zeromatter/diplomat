#[repr(C)]
pub struct StructWithSlices<'a> {
    pub first: &'a [u8],
    pub second: &'a [u16],
}

impl<'a> StructWithSlices<'a> {
    pub fn return_last<'a>(self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { StructWithSlices_return_last(self, write_mut) };
        
        let out_str = write.to_string();
        out_str
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn StructWithSlices_return_last<'a>(this : StructWithSlices, write_mut : &mut crate::DiplomatWrite) -> ();
}