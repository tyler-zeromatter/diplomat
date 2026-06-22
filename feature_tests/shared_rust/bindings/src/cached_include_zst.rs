pub struct CachedIncludeZST;

#[repr(C)]
pub(crate) struct CachedIncludeZSTAbi;

impl CachedIncludeZSTAbi {
    pub(crate) fn from_ffi(self) -> CachedIncludeZST {
        CachedIncludeZST {
            
        }
    }

    pub(crate) fn to_ffi(_this : CachedIncludeZST) -> CachedIncludeZSTAbi {
        CachedIncludeZSTAbi {
            
        }
    }
}

impl From<CachedIncludeZST> for CachedIncludeZSTAbi{
    fn from(value: CachedIncludeZST) -> Self {
        CachedIncludeZSTAbi::to_ffi(value)
    }
}

impl From<CachedIncludeZSTAbi> for CachedIncludeZST{
    fn from(value: CachedIncludeZSTAbi) -> Self {
        CachedIncludeZSTAbi::from_ffi(value)
    }
}

impl CachedIncludeZST {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}