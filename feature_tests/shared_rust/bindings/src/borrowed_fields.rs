use super::Bar;
#[repr(C)]
pub struct BorrowedFields {
    pub a: &[TODO],
    pub b: &[TODO],
    pub c: &[TODO],
}

impl BorrowedFields {
    pub fn from_bar_and_strings(bar : &Bar, dstr16 : &[TODO], utf8_str : &[TODO]) -> BorrowedFields {
        let ret = unsafe { BorrowedFields_from_bar_and_strings(bar, dstr16, utf8_str) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn BorrowedFields_from_bar_and_strings(bar : &Bar, dstr16 : &[TODO], utf8_str : &[TODO]) -> BorrowedFields;

}