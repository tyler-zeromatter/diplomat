use super::RefListParameter;
pub struct RefList;

impl Drop for RefList {
    fn drop(&mut self) {
        unsafe { RefList_destroy(self) }
    }
}

impl RefList {
    pub fn node(data : &'b RefListParameter) -> Box<RefList><'b> {
        let ret = unsafe { RefList_node(data) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn RefList_node(data : &'b RefListParameter) -> Box<RefList><'b>;

    fn RefList_destroy(this : *mut RefList);
}