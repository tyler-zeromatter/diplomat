pub struct MyZst;

#[repr(C)]
pub(crate) struct MyZstAbi;

impl MyZstAbi {
    fn from_ffi(self) -> MyZst{
        MyZst {
            
        }
    }
}

impl MyZst {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}