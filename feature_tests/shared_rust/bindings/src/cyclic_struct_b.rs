use super::CyclicStructA;
pub struct CyclicStructB {
    pub field: u8,
}

#[repr(C)]
pub(crate) struct CyclicStructBAbi {
    
    field : u8,
    
}

impl CyclicStructBAbi {
    fn from_ffi(self) -> CyclicStructB{
        CyclicStructB {
            
                field: self.field,
            
        }
    }
}

impl CyclicStructB {
    pub fn get_a() -> CyclicStructA {
        let ret = unsafe { CyclicStructB_get_a() };
        
        ret.from_ffi()
    
    }

    pub fn get_a_option() -> Option<CyclicStructA> {
        let ret = unsafe { CyclicStructB_get_a_option() };
        
        ret.into_converted_option().map(|ok : CyclicStructAAbi| { ok.from_ffi() })
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn CyclicStructB_get_a() -> CyclicStructAAbi;

    fn CyclicStructB_get_a_option() -> diplomat_runtime::DiplomatOption<CyclicStructAAbi>;
}