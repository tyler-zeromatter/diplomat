use super::RefListParameter;
pub struct RefList;

impl Drop for RefList {
    fn drop(&mut self) {
        unsafe { RefList_destroy(self) }
    }
}

impl RefList {
    pub fn node(data : &RefListParameter) -> Box<RefList> {
        let ret = unsafe { RefList_node(data) };
        ret
    }

    

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn RefList_node(data : &RefListParameter) -> Box<RefList>;

    fn RefList_destroy(this : *mut RefList);

}