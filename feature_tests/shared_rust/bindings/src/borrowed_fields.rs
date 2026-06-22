use super::Bar;
#[repr(C)]
pub struct BorrowedFields<'a> {
    pub a: diplomat_runtime::DiplomatStr16Slice::<'a>,
    pub b: diplomat_runtime::DiplomatStrSlice::<'a>,
    pub c: diplomat_runtime::DiplomatUtf8StrSlice::<'a>,
}

impl<'a> BorrowedFields<'a> {
    pub fn from_bar_and_strings<'x>(bar : &'x Bar<'x, 'x>, dstr16 : &'x [u16], utf8_str : &'x str) -> BorrowedFields<'x> {
        let ret = unsafe { BorrowedFields_from_bar_and_strings(bar, dstr16.into(), utf8_str.into()) };
        
        ret

    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn BorrowedFields_from_bar_and_strings<'x>(bar : &'x Bar<'x, 'x>, dstr16 : diplomat_runtime::DiplomatStr16Slice::<'x>, utf8_str : diplomat_runtime::DiplomatUtf8StrSlice::<'x>) -> BorrowedFields<'x>;
}