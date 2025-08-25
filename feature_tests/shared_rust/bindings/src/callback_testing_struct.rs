#[repr(C)]
pub struct CallbackTestingStruct;

impl CallbackTestingStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}