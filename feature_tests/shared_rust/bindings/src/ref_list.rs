use super::RefListParameter;
pub struct RefList<'a>;

impl Drop for RefList {
    fn drop(&mut self) {
        unsafe { RefList_destroy(self) }
    }
}

impl<'a> RefList<'a> {
    pub fn node<'b>(data : &'b RefListParameter) -> Box<RefList><'b> {
        let ret = unsafe { RefList_node(data) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn RefList_node<'b>(data : &'b RefListParameter) -> Box<RefList><'b>;

    fn RefList_destroy(this : *mut RefList);
}