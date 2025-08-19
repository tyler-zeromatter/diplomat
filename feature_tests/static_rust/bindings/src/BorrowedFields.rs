#[repr(C)]
pub struct BorrowedFields {

}

impl BorrowedFields {
    fn from_bar_and_strings() {
        unsafe { BorrowedFields_from_bar_and_strings() }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn BorrowedFields_from_bar_and_strings();

}