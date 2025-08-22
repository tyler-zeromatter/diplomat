use super::RefListParameter;
pub struct RefList;

impl RefList {
    pub fn node(data : &RefListParameter) -> Box<RefList> {
        let ret = unsafe { RefList_node(data) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RefList_node(data : &RefListParameter) -> Box<RefList>;

}