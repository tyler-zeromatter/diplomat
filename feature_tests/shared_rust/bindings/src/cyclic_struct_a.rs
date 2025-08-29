use super::CyclicStructB;
pub struct CyclicStructA {
    pub a: CyclicStructB,
}

#[repr(C)]
pub(crate) struct CyclicStructAAbi {
    
    a : CyclicStructBAbi,
    
}

impl CyclicStructAAbi {
    fn from_ffi(self) -> CyclicStructA{
        CyclicStructA {
            
                a: self.a.from_ffi(),
            
        }
    }
}

impl CyclicStructA {
    pub fn get_b() -> CyclicStructB {
        let ret = unsafe { CyclicStructA_get_b() };
        
        ret.from_ffi()
    
    }

    pub fn cyclic_out(self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { CyclicStructA_cyclic_out(self, write_mut) };
        
        let out_str = write.to_string();
        out_str
    
    }

    pub fn double_cyclic_out(self, cyclic_struct_a : CyclicStructA) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { CyclicStructA_double_cyclic_out(self, cyclic_struct_a, write_mut) };
        
        let out_str = write.to_string();
        out_str
    
    }

    pub fn getter_out(self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { CyclicStructA_getter_out(self, write_mut) };
        
        let out_str = write.to_string();
        out_str
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn CyclicStructA_get_b() -> CyclicStructBAbi;

    fn CyclicStructA_cyclic_out(this : CyclicStructA, write_mut : &mut crate::DiplomatWrite) -> ();

    fn CyclicStructA_double_cyclic_out(this : CyclicStructA, cyclic_struct_a : CyclicStructAAbi, write_mut : &mut crate::DiplomatWrite) -> ();

    fn CyclicStructA_getter_out(this : CyclicStructA, write_mut : &mut crate::DiplomatWrite) -> ();
}