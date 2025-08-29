use super::CyclicStructA;
use super::cyclic_struct_a::CyclicStructAAbi;
pub struct CyclicStructC {
    pub a: CyclicStructA,
}

#[repr(C)]
pub(crate) struct CyclicStructCAbi {
    a : CyclicStructAAbi,
}

impl CyclicStructCAbi {
    pub(crate) fn from_ffi(self) -> CyclicStructC{
        CyclicStructC {
            
            a: self.a.from_ffi(),
            
        }
    }

    pub (crate) fn to_ffi(this : CyclicStructC) -> CyclicStructCAbi{
        CyclicStructCAbi {
            
            a : this.a.into(),
            
        }
    }
}

impl From<CyclicStructC> for CyclicStructCAbi{
    fn from(value: CyclicStructC) -> Self {
        CyclicStructCAbi::to_ffi(value)
    }
}

impl From<CyclicStructCAbi> for CyclicStructC{
    fn from(value: CyclicStructCAbi) -> Self {
        CyclicStructCAbi::from_ffi(value)
    }
}

impl CyclicStructC {
    pub fn takes_nested_parameters(c : CyclicStructC) -> CyclicStructC {
        let ret = unsafe { CyclicStructC_takes_nested_parameters(c.into()) };
        
        ret.from_ffi()
    
    }

    pub fn cyclic_out(self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        unsafe { CyclicStructC_cyclic_out(self.into(), write_mut) };
        
        let out_str = write.to_string();
        out_str
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn CyclicStructC_takes_nested_parameters(c : CyclicStructCAbi) -> CyclicStructCAbi;

    fn CyclicStructC_cyclic_out(this : CyclicStructCAbi, write_mut : &mut crate::DiplomatWrite) -> ();
}