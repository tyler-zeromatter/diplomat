use super::OptionOpaque;
use super::OptionOpaqueChar;
pub(super) struct OptionStruct {
    pub a: Option<Box<OptionOpaque>>,
    pub b: Option<Box<OptionOpaqueChar>>,
    pub c: u32,
    pub d: Box<OptionOpaque>,
}

#[repr(C)]
pub(crate) struct OptionStructAbi {
    
    a : Option<Box<OptionOpaque>>,
    
    b : Option<Box<OptionOpaqueChar>>,
    
    c : u32,
    
    d : Box<OptionOpaque>,
    
}

impl OptionStructAbi {
    fn from_ffi(self) -> OptionStruct{
        OptionStruct {
            
                a: self.a,
            
                b: self.b,
            
                c: self.c,
            
                d: self.d,
            
        }
    }
}

impl OptionStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}