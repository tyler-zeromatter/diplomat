use super::Bar;
#[repr(C)]
pub struct BorrowedFields<'a> {
    pub a: &'a [u16],
    pub b: &'a [u8],
    pub c: &'a String,
}

impl<'a> BorrowedFields<'a> {
    pub fn from_bar_and_strings<'x>(bar : &'x Bar<'x, 'x>, dstr16 : &'x [u16], utf8_str : &'x String) -> BorrowedFields<'x> {
        let ret = unsafe { BorrowedFields_from_bar_and_strings(bar, dstr16.into(), utf8_str.into()) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn BorrowedFields_from_bar_and_strings<'x>(bar : &'x Bar<'x, 'x>, dstr16 : &'x diplomat_runtime::DiplomatStrSlice, utf8_str : &'x diplomat_runtime::DiplomatStrSlice) -> BorrowedFields<'x>;
}