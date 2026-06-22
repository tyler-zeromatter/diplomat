pub struct RenamedFeatureTest;

#[repr(C)]
pub(crate) struct RenamedFeatureTestAbi;

impl RenamedFeatureTestAbi {
    pub(crate) fn from_ffi(self) -> RenamedFeatureTest {
        RenamedFeatureTest {
            
        }
    }

    pub(crate) fn to_ffi(_this : RenamedFeatureTest) -> RenamedFeatureTestAbi {
        RenamedFeatureTestAbi {
            
        }
    }
}

impl From<RenamedFeatureTest> for RenamedFeatureTestAbi{
    fn from(value: RenamedFeatureTest) -> Self {
        RenamedFeatureTestAbi::to_ffi(value)
    }
}

impl From<RenamedFeatureTestAbi> for RenamedFeatureTest{
    fn from(value: RenamedFeatureTestAbi) -> Self {
        RenamedFeatureTestAbi::from_ffi(value)
    }
}

impl RenamedFeatureTest {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}