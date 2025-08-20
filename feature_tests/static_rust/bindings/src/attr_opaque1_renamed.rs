use super::RenamedAttrEnum;
use super::Unnamespaced;
pub struct AttrOpaque1Renamed;

impl AttrOpaque1Renamed {
    fn new() -> AttrOpaque1Renamed {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_new() }
    }

    fn mac_test() -> i32 {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_mac_test() }
    }

    fn hello() -> i32 {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_hello() }
    }

    fn method(&self) -> u8 {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_method(self) }
    }

    fn abirenamed(&self) -> u8 {
            // TODO: writeable conversions.
        unsafe { renamed_on_abi_only(self) }
    }

    fn use_unnamespaced(&self, _un : Unnamespaced) {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_use_unnamespaced(self, _un) }
    }

    fn use_namespaced(&self, _n : RenamedAttrEnum) {
            // TODO: writeable conversions.
        unsafe { namespace_AttrOpaque1_use_namespaced(self, _n) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_AttrOpaque1_new() -> AttrOpaque1Renamed;

    fn namespace_AttrOpaque1_mac_test() -> i32;

    fn namespace_AttrOpaque1_hello() -> i32;

    fn namespace_AttrOpaque1_method(this: &AttrOpaque1Renamed) -> u8;

    fn renamed_on_abi_only(this: &AttrOpaque1Renamed) -> u8;

    fn namespace_AttrOpaque1_use_unnamespaced(this: &AttrOpaque1Renamed, _un : Unnamespaced);

    fn namespace_AttrOpaque1_use_namespaced(this: &AttrOpaque1Renamed, _n : RenamedAttrEnum);

}