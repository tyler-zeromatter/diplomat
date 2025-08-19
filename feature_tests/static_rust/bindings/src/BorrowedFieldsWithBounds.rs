#[repr(C)]
pub struct BorrowedFieldsWithBounds {

}

impl BorrowedFieldsWithBounds {
    fn from_foo_and_strings() {
        unsafe { BorrowedFieldsWithBounds_from_foo_and_strings() }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn BorrowedFieldsWithBounds_from_foo_and_strings();

}