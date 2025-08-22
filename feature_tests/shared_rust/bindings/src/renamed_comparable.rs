pub struct RenamedComparable;

impl Drop for RenamedComparable {
    fn drop(&mut self) {
        unsafe { RenamedComparable_destroy(self) }
    }
}

impl RenamedComparable {
}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_Comparable_destroy(this : *mut RenamedComparable);
}