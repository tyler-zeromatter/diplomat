#[repr(C)]
pub struct RenamedRenamedCachedIncludeZST;

impl RenamedRenamedCachedIncludeZST {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}