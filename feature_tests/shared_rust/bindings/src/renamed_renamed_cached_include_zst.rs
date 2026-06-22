pub struct RenamedRenamedCachedIncludeZST;

#[repr(C)]
pub(crate) struct RenamedRenamedCachedIncludeZSTAbi;

impl RenamedRenamedCachedIncludeZSTAbi {
    pub(crate) fn from_ffi(self) -> RenamedRenamedCachedIncludeZST {
        RenamedRenamedCachedIncludeZST {
            
        }
    }

    pub(crate) fn to_ffi(_this : RenamedRenamedCachedIncludeZST) -> RenamedRenamedCachedIncludeZSTAbi {
        RenamedRenamedCachedIncludeZSTAbi {
            
        }
    }
}

impl From<RenamedRenamedCachedIncludeZST> for RenamedRenamedCachedIncludeZSTAbi{
    fn from(value: RenamedRenamedCachedIncludeZST) -> Self {
        RenamedRenamedCachedIncludeZSTAbi::to_ffi(value)
    }
}

impl From<RenamedRenamedCachedIncludeZSTAbi> for RenamedRenamedCachedIncludeZST{
    fn from(value: RenamedRenamedCachedIncludeZSTAbi) -> Self {
        RenamedRenamedCachedIncludeZSTAbi::from_ffi(value)
    }
}

impl RenamedRenamedCachedIncludeZST {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}