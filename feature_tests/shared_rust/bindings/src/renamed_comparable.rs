

pub struct RenamedComparable;

impl Drop for RenamedComparable {
    fn drop(&mut self) {
        unsafe { namespace_Comparable_destroy(self) }
    }
}

impl RenamedComparable {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_Comparable_destroy(this : *mut RenamedComparable);
}