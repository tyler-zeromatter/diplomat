use super::Bar;
#[repr(C)]
pub struct BorrowedFields {
    pub a: &[u16],
    pub b: &[u8],
    pub c: &String,
}

impl BorrowedFields {
    pub fn from_bar_and_strings(bar : &Bar, dstr16 : &[u16], utf8_str : &String) -> BorrowedFields {
        let ret = unsafe { BorrowedFields_from_bar_and_strings(bar, dstr16.into(), utf8_str.into()) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn BorrowedFields_from_bar_and_strings(bar : &Bar, dstr16 : diplomat_runtime::DiplomatStrSlice, utf8_str : diplomat_runtime::DiplomatStrSlice) -> BorrowedFields;

}