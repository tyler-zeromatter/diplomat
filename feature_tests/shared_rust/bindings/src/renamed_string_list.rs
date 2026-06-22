pub struct RenamedStringList;

impl Drop for RenamedStringList {
    fn drop(&mut self) {
        unsafe { namespace_StringList_destroy(self) }
    }
}

impl RenamedStringList {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_StringList_destroy(this : *mut RenamedStringList);
}