#[repr(C)]
pub struct NestedBorrowedFields {

}

impl NestedBorrowedFields {
    fn from_bar_and_foo_and_strings() {
        unsafe { NestedBorrowedFields_from_bar_and_foo_and_strings() }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn NestedBorrowedFields_from_bar_and_foo_and_strings();

}