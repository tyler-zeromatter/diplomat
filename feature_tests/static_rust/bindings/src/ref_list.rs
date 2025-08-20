use super::RefListParameter;
pub struct RefList;

impl RefList {
    fn node(data : RefListParameter) -> RefList {
            // TODO: writeable conversions.
        unsafe { RefList_node(data) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RefList_node(data : RefListParameter) -> RefList;

}