use super::AttrOpaque1Renamed;
use super::RenamedAttrEnum;
pub struct Unnamespaced;

impl Unnamespaced {
    fn make(_e : RenamedAttrEnum) -> Box<Unnamespaced> {
            // TODO: writeable conversions.
        unsafe { namespace_Unnamespaced_make(_e) }
    }

    fn use_namespaced(&self, _n : &AttrOpaque1Renamed) {
            // TODO: writeable conversions.
        unsafe { namespace_Unnamespaced_use_namespaced(self, _n) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_Unnamespaced_make(_e : RenamedAttrEnum) -> Box<Unnamespaced>;

    fn namespace_Unnamespaced_use_namespaced(this: &Unnamespaced, _n : &AttrOpaque1Renamed);

}