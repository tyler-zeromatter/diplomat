use super::RenamedAttrEnum;
use super::Unnamespaced;
pub struct AttrOpaque1Renamed;

impl AttrOpaque1Renamed {
    pub fn new() -> Box<AttrOpaque1Renamed> {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_new() }
    }

    pub fn mac_test() -> i32 {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_mac_test() }
    }

    pub fn hello() -> i32 {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_hello() }
    }

    pub fn method(&self) -> u8 {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_method(self) }
    }

    pub fn abirenamed(&self) -> u8 {
            // TODO: writeable conversions.
        unsafe { renamed_on_abi_only(self) }
    }

    pub fn use_unnamespaced(&self, _un : &Unnamespaced) {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_use_unnamespaced(self, _un) }
    }

    pub fn use_namespaced(&self, _n : RenamedAttrEnum) {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_use_namespaced(self, _n) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_AttrOpaque1_new() -> Box<AttrOpaque1Renamed>;

    fn namespace_AttrOpaque1_mac_test() -> i32;

    fn namespace_AttrOpaque1_hello() -> i32;

    fn namespace_AttrOpaque1_method(this: &AttrOpaque1Renamed) -> u8;

    fn renamed_on_abi_only(this: &AttrOpaque1Renamed) -> u8;

    fn namespace_AttrOpaque1_use_unnamespaced(this: &AttrOpaque1Renamed, _un : &Unnamespaced);

    fn namespace_AttrOpaque1_use_namespaced(this: &AttrOpaque1Renamed, _n : RenamedAttrEnum);

}