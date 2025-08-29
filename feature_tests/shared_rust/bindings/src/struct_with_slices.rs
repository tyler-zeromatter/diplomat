pub struct StructWithSlices<'a> {
    pub first: &'a [u8],
    pub second: &'a [u16],
}

#[repr(C)]
pub(crate) struct StructWithSlicesAbi<'a> {
    first : diplomat_runtime::DiplomatStrSlice<'a>,
    second : &'a [u16],
}

impl<'a> StructWithSlicesAbi<'a> {
    pub(crate) fn from_ffi(self) -> StructWithSlices<'a>{
        StructWithSlices {
            
            first: self.first.into(),
            
            second: self.second,
            
        }
    }

    pub (crate) fn to_ffi(this : StructWithSlices<'a>) -> StructWithSlicesAbi<'a>{
        StructWithSlicesAbi {
            
            first : this.first.into(),
            
            second : this.second,
            
        }
    }
}

impl<'a> From<StructWithSlices<'a>> for StructWithSlicesAbi<'a>{
    fn from(value: StructWithSlices<'a>) -> Self {
        StructWithSlicesAbi::to_ffi(value)
    }
}

impl<'a> From<StructWithSlicesAbi<'a>> for StructWithSlices<'a>{
    fn from(value: StructWithSlicesAbi<'a>) -> Self {
        value.from_ffi()
    }
}

impl<'a> StructWithSlices<'a> {
    pub fn return_last(self) -> String {
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