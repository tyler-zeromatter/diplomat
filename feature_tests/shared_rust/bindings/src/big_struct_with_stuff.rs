pub struct BigStructWithStuff;

#[repr(C)]
pub(crate) struct BigStructWithStuffAbi;

impl BigStructWithStuffAbi {
    pub(crate) fn from_ffi(self) -> BigStructWithStuff{
        BigStructWithStuff {
            
        }
    }

    pub (crate) fn to_ffi(this : BigStructWithStuff) -> BigStructWithStuffAbi{
        BigStructWithStuffAbi {
            
        }
    }
}

impl From<BigStructWithStuff> for BigStructWithStuffAbi{
    fn from(value: BigStructWithStuff) -> Self {
        BigStructWithStuffAbi::to_ffi(value)
    }
}

impl From<BigStructWithStuffAbi> for BigStructWithStuff{
    fn from(value: BigStructWithStuffAbi) -> Self {
        BigStructWithStuffAbi::from_ffi(value)
    }
}

impl BigStructWithStuff {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}