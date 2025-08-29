pub struct BigStructWithStuff;

#[repr(C)]
pub(crate) struct BigStructWithStuffAbi;

impl BigStructWithStuffAbi {
    fn from_ffi(self) -> BigStructWithStuff{
        BigStructWithStuff {
            
        }
    }
}

impl BigStructWithStuff {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}