#[repr(C)]
pub struct CallbackTestingStruct;

impl CallbackTestingStruct {
}

#[link(name = "somelib")]
unsafe extern "C" {}