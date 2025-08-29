pub struct RenamedStructWithAttrs {
    pub a: bool,
    pub b: u32,
}

#[repr(C)]
pub(crate) struct RenamedStructWithAttrsAbi {
    
    a : bool,
    
    b : u32,
    
}

impl RenamedStructWithAttrsAbi {
    fn from_ffi(self) -> RenamedStructWithAttrs{
        RenamedStructWithAttrs {
            
                a: self.a,
            
                b: self.b,
            
        }
    }
}

impl RenamedStructWithAttrs {
    pub fn new_fallible(a : bool, b : u32) -> Result<RenamedStructWithAttrs, ()> {
        let ret = unsafe { namespace_StructWithAttrs_new_fallible(a, b) };
        
        ret.to_result().map(|ok : RenamedStructWithAttrsAbi| { ok.from_ffi() })
    
    }

    pub fn c(self) -> u32 {
        let ret = unsafe { namespace_StructWithAttrs_c(self) };
        
        ret
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_StructWithAttrs_new_fallible(a : bool, b : u32) -> crate::DiplomatResult<RenamedStructWithAttrsAbi, ()>;

    fn namespace_StructWithAttrs_c(this : RenamedStructWithAttrs) -> u32;
}