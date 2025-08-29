pub struct ErrorStruct {
    pub i: i32,
    pub j: i32,
}

#[repr(C)]
pub(crate) struct ErrorStructAbi {
    i : i32,
    j : i32,
}

impl ErrorStructAbi {
    pub(crate) fn from_ffi(self) -> ErrorStruct{
        ErrorStruct {
            
            i: self.i,
            
            j: self.j,
            
        }
    }

    pub (crate) fn to_ffi(this : ErrorStruct) -> ErrorStructAbi{
        ErrorStructAbi {
            
            i : this.i,
            
            j : this.j,
            
        }
    }
}

impl From<ErrorStruct> for ErrorStructAbi{
    fn from(value: ErrorStruct) -> Self {
        ErrorStructAbi::to_ffi(value)
    }
}

impl From<ErrorStructAbi> for ErrorStruct{
    fn from(value: ErrorStructAbi) -> Self {
        ErrorStructAbi::from_ffi(value)
    }
}

impl ErrorStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}