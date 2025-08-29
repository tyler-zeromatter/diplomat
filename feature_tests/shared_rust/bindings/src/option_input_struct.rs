use super::OptionEnum;
pub struct OptionInputStruct {
    pub a: Option<u8>,
    pub b: Option<diplomat_runtime::DiplomatChar>,
    pub c: Option<OptionEnum>,
}

#[repr(C)]
pub(crate) struct OptionInputStructAbi {
    
    a : diplomat_runtime::DiplomatOption<u8>,
    
    b : diplomat_runtime::DiplomatOption<diplomat_runtime::DiplomatChar>,
    
    c : diplomat_runtime::DiplomatOption<OptionEnum>,
    
}

impl OptionInputStructAbi {
    fn from_ffi(self) -> OptionInputStruct{
        OptionInputStruct {
            
                a: self.a,
            
                b: self.b,
            
                c: self.c,
            
        }
    }
}

impl OptionInputStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}