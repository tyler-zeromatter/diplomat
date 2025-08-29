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
    pub(crate) fn from_ffi(self) -> OptionStruct{
        OptionStruct {
            
            a: self.a,
            
            b: self.b,
            
            c: self.c,
            
            d: self.d,
            
        }
    }

    pub (crate) fn to_ffi(this : OptionStruct) -> OptionStructAbi{
        OptionStructAbi {
            
            a : this.a,
            
            b : this.b,
            
            c : this.c,
            
            d : this.d,
            
        }
    }
}

impl From<OptionStruct> for OptionStructAbi{
    fn from(value: OptionStruct) -> Self {
        OptionStructAbi::to_ffi(value)
    }
}

impl From<OptionStructAbi> for OptionStruct{
    fn from(value: OptionStructAbi) -> Self {
        OptionStructAbi::from_ffi(value)
    }
}

impl OptionStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}