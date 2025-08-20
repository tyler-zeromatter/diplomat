use super::RefListParameter;
pub struct RefList;

impl RefList {
    pub fn node(data : &RefListParameter) -> Box<RefList> {
            // TODO: writeable conversions.
        unsafe { RefList_node(data) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RefList_node(data : &RefListParameter) -> Box<RefList>;

}