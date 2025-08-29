pub struct StructArithmetic;

#[repr(C)]
pub(crate) struct StructArithmeticAbi;

impl StructArithmeticAbi {
    pub(crate) fn from_ffi(self) -> StructArithmetic {
        StructArithmetic {
            
        }
    }

    pub(crate) fn to_ffi(_this : StructArithmetic) -> StructArithmeticAbi {
        StructArithmeticAbi {
            
        }
    }
}

impl From<StructArithmetic> for StructArithmeticAbi{
    fn from(value: StructArithmetic) -> Self {
        StructArithmeticAbi::to_ffi(value)
    }
}

impl From<StructArithmeticAbi> for StructArithmetic{
    fn from(value: StructArithmeticAbi) -> Self {
        StructArithmeticAbi::from_ffi(value)
    }
}

impl StructArithmetic {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}