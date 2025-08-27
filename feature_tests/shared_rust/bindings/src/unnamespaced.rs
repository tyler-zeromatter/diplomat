use super::AttrOpaque1Renamed;
use super::RenamedAttrEnum;
pub struct Unnamespaced;

impl Drop for Unnamespaced {
    fn drop(&mut self) {
        unsafe { namespace_Unnamespaced_destroy(self) }
    }
}

impl Unnamespaced {
    pub fn make(_e : RenamedAttrEnum) -> Box<Unnamespaced> {
        let ret = unsafe { namespace_Unnamespaced_make(_e) };
        ret
    }

    pub fn use_namespaced<'anon_0, 'anon_1>(&self, _n : &'anon_1 AttrOpaque1Renamed) {
        let ret = unsafe { namespace_Unnamespaced_use_namespaced(self, _n) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_Unnamespaced_make(_e : RenamedAttrEnum) -> Box<Unnamespaced>;

    fn namespace_Unnamespaced_use_namespaced<'anon_0, 'anon_1>(this: &Unnamespaced, _n : &'anon_1 AttrOpaque1Renamed);

    fn namespace_Unnamespaced_destroy(this : *mut Unnamespaced);
}