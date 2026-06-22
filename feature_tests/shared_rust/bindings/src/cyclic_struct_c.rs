use super::CyclicStructA;
use super::cyclic_struct_a::CyclicStructAAbi;
#[repr(C)]
pub struct CyclicStructC {
    pub a: CyclicStructAAbi,
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