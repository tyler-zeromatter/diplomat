#[repr(C)]
pub struct CachedIncludeZST;

impl CachedIncludeZST {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}