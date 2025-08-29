use super::CyclicStructA;
use super::cyclic_struct_a::CyclicStructAAbi;
pub struct CyclicStructB {
    pub field: u8,
}

#[repr(C)]
pub(crate) struct CyclicStructBAbi {
    field : u8,
}

impl CyclicStructBAbi {
    pub(crate) fn from_ffi(self) -> CyclicStructB{
        CyclicStructB {
            
            field: self.field,
            
        }
    }

    pub (crate) fn to_ffi(this : CyclicStructB) -> CyclicStructBAbi{
        CyclicStructBAbi {
            
            field : this.field,
            
        }
    }
}

impl From<CyclicStructB> for CyclicStructBAbi{
    fn from(value: CyclicStructB) -> Self {
        CyclicStructBAbi::to_ffi(value)
    }
}

impl From<CyclicStructBAbi> for CyclicStructB{
    fn from(value: CyclicStructBAbi) -> Self {
        CyclicStructBAbi::from_ffi(value)
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