pub struct StructArithmetic;

#[repr(C)]
pub(crate) struct StructArithmeticAbi;

impl StructArithmeticAbi {
    fn from_ffi(self) -> StructArithmetic{
        StructArithmetic {
            
        }
    }
}

impl StructArithmetic {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}