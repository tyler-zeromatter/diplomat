use super::RenamedAttrEnum;
use super::Unnamespaced;


pub struct AttrOpaque1Renamed;

impl Drop for AttrOpaque1Renamed {
    fn drop(&mut self) {
        unsafe { namespace_AttrOpaque1_destroy(self) }
    }
}

impl AttrOpaque1Renamed {
    pub fn new() -> Box<AttrOpaque1Renamed> {
        let ret = unsafe { namespace_AttrOpaque1_new() };
        ret
    }

    pub fn mac_test() -> i32 {
        let ret = unsafe { namespace_AttrOpaque1_mac_test() };
        ret
    }

    pub fn hello() -> i32 {
        let ret = unsafe { namespace_AttrOpaque1_hello() };
        ret
    }

    pub fn method<'anon_0>(&self) -> u8 {
        let ret = unsafe { namespace_AttrOpaque1_method(self) };
        ret
    }

    pub fn abirenamed<'anon_0>(&self) -> u8 {
        let ret = unsafe { renamed_on_abi_only(self) };
        ret
    }

    pub fn use_unnamespaced<'anon_0, 'anon_1>(&self, _un : &'anon_1 Unnamespaced) {
        let ret = unsafe { namespace_AttrOpaque1_use_unnamespaced(self, _un) };
        ret
    }

    pub fn use_namespaced<'anon_0>(&self, _n : RenamedAttrEnum) {
        let ret = unsafe { namespace_AttrOpaque1_use_namespaced(self, _n) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_AttrOpaque1_new() -> Box<AttrOpaque1Renamed>;

    fn namespace_AttrOpaque1_mac_test() -> i32;

    fn namespace_AttrOpaque1_hello() -> i32;

    fn namespace_AttrOpaque1_method<'anon_0>(this: &AttrOpaque1Renamed) -> u8;

    fn renamed_on_abi_only<'anon_0>(this: &AttrOpaque1Renamed) -> u8;

    fn namespace_AttrOpaque1_use_unnamespaced<'anon_0, 'anon_1>(this: &AttrOpaque1Renamed, _un : &'anon_1 Unnamespaced);

    fn namespace_AttrOpaque1_use_namespaced<'anon_0>(this: &AttrOpaque1Renamed, _n : RenamedAttrEnum);

    fn namespace_AttrOpaque1_destroy(this : *mut AttrOpaque1Renamed);
}