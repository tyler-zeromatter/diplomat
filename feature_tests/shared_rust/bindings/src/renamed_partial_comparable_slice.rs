pub struct RenamedPartialComparableSlice;

#[repr(C)]
pub(crate) struct RenamedPartialComparableSliceAbi;

impl RenamedPartialComparableSliceAbi {
    pub(crate) fn from_ffi(self) -> RenamedPartialComparableSlice {
        RenamedPartialComparableSlice {
            
        }
    }

    pub(crate) fn to_ffi(_this : RenamedPartialComparableSlice) -> RenamedPartialComparableSliceAbi {
        RenamedPartialComparableSliceAbi {
            
        }
    }
}

impl From<RenamedPartialComparableSlice> for RenamedPartialComparableSliceAbi{
    fn from(value: RenamedPartialComparableSlice) -> Self {
        RenamedPartialComparableSliceAbi::to_ffi(value)
    }
}

impl From<RenamedPartialComparableSliceAbi> for RenamedPartialComparableSlice{
    fn from(value: RenamedPartialComparableSliceAbi) -> Self {
        RenamedPartialComparableSliceAbi::from_ffi(value)
    }
}

impl RenamedPartialComparableSlice {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}