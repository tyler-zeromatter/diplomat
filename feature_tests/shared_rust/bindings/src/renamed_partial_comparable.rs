pub struct RenamedPartialComparable;

impl Drop for RenamedPartialComparable {
    fn drop(&mut self) {
        unsafe { namespace_PartialComparable_destroy(self) }
    }
}

impl RenamedPartialComparable {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_PartialComparable_destroy(this : *mut RenamedPartialComparable);
}