pub struct RenamedMyIterable;

impl Drop for RenamedMyIterable {
    fn drop(&mut self) {
        unsafe { RenamedMyIterable_destroy(self) }
    }
}

impl RenamedMyIterable {
}

#[link(name = "somelib")]
unsafe extern "C" {
}