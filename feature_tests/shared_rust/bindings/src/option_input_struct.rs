use super::OptionEnum;
pub struct OptionInputStruct {
    pub a: Option<u8>,
    pub b: Option<char>,
    pub c: Option<OptionEnum>,
}

#[repr(C)]
pub(crate) struct OptionInputStructAbi {
    a : diplomat_runtime::DiplomatOption<u8>,
    b : diplomat_runtime::DiplomatOption<char>,
    c : diplomat_runtime::DiplomatOption<OptionEnum>,
}

impl OptionInputStructAbi {
    pub(crate) fn from_ffi(self) -> OptionInputStruct{
        OptionInputStruct {
            
            a: self.a.into_converted_option(),
            
            b: self.b.into_converted_option(),
            
            c: self.c.into_converted_option(),
            
        }
    }

    pub (crate) fn to_ffi(this : OptionInputStruct) -> OptionInputStructAbi{
        OptionInputStructAbi {
            
            a : this.a.into(),
            
            b : this.b.into(),
            
            c : this.c.into(),
            
        }
    }
}

impl From<OptionInputStruct> for OptionInputStructAbi{
    fn from(value: OptionInputStruct) -> Self {
        OptionInputStructAbi::to_ffi(value)
    }
}

impl From<OptionInputStructAbi> for OptionInputStruct{
    fn from(value: OptionInputStructAbi) -> Self {
        OptionInputStructAbi::from_ffi(value)
    }
}

impl OptionInputStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}