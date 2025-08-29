pub struct MyZst;

#[repr(C)]
pub(crate) struct MyZstAbi;

impl MyZstAbi {
    pub(crate) fn from_ffi(self) -> MyZst{
        MyZst {
            
        }
    }

    pub (crate) fn to_ffi(this : MyZst) -> MyZstAbi{
        MyZstAbi {
            
        }
    }
}

impl From<MyZst> for MyZstAbi{
    fn from(value: MyZst) -> Self {
        MyZstAbi::to_ffi(value)
    }
}

impl From<MyZstAbi> for MyZst{
    fn from(value: MyZstAbi) -> Self {
        MyZstAbi::from_ffi(value)
    }
}

impl MyZst {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}