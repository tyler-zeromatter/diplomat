#[repr(C)]
pub struct BorrowedFields {

}

impl BorrowedFields {
    fn from_bar_and_strings(bar : TODO(), dstr16 : TODO(), utf8_str : TODO()) {
        unsafe { BorrowedFields_from_bar_and_strings(bar, dstr16, utf8_str) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn BorrowedFields_from_bar_and_strings(bar : TODO(), dstr16 : TODO(), utf8_str : TODO());

}