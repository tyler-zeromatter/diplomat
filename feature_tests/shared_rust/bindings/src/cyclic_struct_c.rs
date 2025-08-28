use super::CyclicStructA;
#[repr(C)]
pub struct CyclicStructC {
    pub a: CyclicStructA,
}

impl CyclicStructC {
    pub fn takes_nested_parameters(c : CyclicStructC) -> CyclicStructC {
        let ret = unsafe { CyclicStructC_takes_nested_parameters(c) };
        
        ret
    }

    pub fn cyclic_out(self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { CyclicStructC_cyclic_out(self, write_mut) };
        
        let out_str = write.to_string();
        out_str
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn CyclicStructC_takes_nested_parameters(c : CyclicStructC) -> CyclicStructC;

    fn CyclicStructC_cyclic_out(this : CyclicStructC, write_mut : &mut crate::DiplomatWrite) -> ();
}