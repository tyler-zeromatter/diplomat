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
    fn from_ffi(self) -> ErrorStruct{
        ErrorStruct {
            
                i: self.i,
            
                j: self.j,
            
        }
    }
}

impl ErrorStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}