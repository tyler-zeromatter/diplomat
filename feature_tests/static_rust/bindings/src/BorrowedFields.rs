use super::Bar;
#[repr(C)]
pub struct BorrowedFields {

}

impl BorrowedFields {
    fn from_bar_and_strings(bar : Bar, dstr16 : TODO(), utf8_str : TODO()) -> BorrowedFields {
        unsafe { BorrowedFields_from_bar_and_strings(bar, dstr16, utf8_str) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn BorrowedFields_from_bar_and_strings(bar : Bar, dstr16 : TODO(), utf8_str : TODO()) -> BorrowedFields;

}