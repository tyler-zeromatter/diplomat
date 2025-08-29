#[repr(C)]
pub enum RenamedAttrEnum {
    A = 0, 
    B = 1, 
    Renamed = 2
}

impl RenamedAttrEnum {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}