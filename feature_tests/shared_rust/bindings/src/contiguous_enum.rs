#[repr(C)]
pub enum ContiguousEnum {
    C = 0, 
    D = 1, 
    E = 2, 
    F = 3
}

impl ContiguousEnum {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}