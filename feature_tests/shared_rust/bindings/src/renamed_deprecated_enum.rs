#[repr(C)]
pub enum RenamedDeprecatedEnum {
    A = 0
}

impl RenamedDeprecatedEnum {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}