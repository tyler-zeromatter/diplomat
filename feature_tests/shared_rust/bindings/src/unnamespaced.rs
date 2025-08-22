use super::AttrOpaque1Renamed;
use super::RenamedAttrEnum;
pub struct Unnamespaced;

impl Drop for Unnamespaced {
    fn drop(&mut self) {
        unsafe { Unnamespaced_destroy(self) }
    }
}

impl Unnamespaced {
    pub fn make(_e : RenamedAttrEnum) -> Box<Unnamespaced> {
        let ret = unsafe { namespace_Unnamespaced_make(_e) };
        ret
    }

    pub fn use_namespaced(&self, _n : &AttrOpaque1Renamed) {
        let ret = unsafe { namespace_Unnamespaced_use_namespaced(self, _n) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_Unnamespaced_make(_e : RenamedAttrEnum) -> Box<Unnamespaced>;

    fn namespace_Unnamespaced_use_namespaced(this: &Unnamespaced, _n : &AttrOpaque1Renamed);

    fn namespace_Unnamespaced_destroy(this : *mut Unnamespaced);
}