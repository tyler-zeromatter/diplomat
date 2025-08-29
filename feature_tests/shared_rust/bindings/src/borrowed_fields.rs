use super::Bar;
pub struct BorrowedFields<'a> {
    pub a: &'a [u16],
    pub b: &'a [u8],
    pub c: &'a str,
}

#[repr(C)]
pub(crate) struct BorrowedFieldsAbi<'a> {
    a : diplomat_runtime::DiplomatStr16Slice<'a>,
    b : diplomat_runtime::DiplomatStrSlice<'a>,
    c : diplomat_runtime::DiplomatStrSlice<'a>,
}

impl<'a> BorrowedFieldsAbi<'a> {
    pub(crate) fn from_ffi(self) -> BorrowedFields<'a>{
        BorrowedFields {
            
            a: self.a.into(),
            
            b: self.b.into(),
            
            c: unsafe { str::from_utf8_unchecked(self.c.into()).into()},
            
        }
    }

    pub (crate) fn to_ffi(this : BorrowedFields<'a>) -> BorrowedFieldsAbi<'a>{
        BorrowedFieldsAbi {
            
            a : this.a.into(),
            
            b : this.b.into(),
            
            c : this.c.as_bytes().into(),
            
        }
    }
}

impl<'a> From<BorrowedFields<'a>> for BorrowedFieldsAbi<'a>{
    fn from(value: BorrowedFields<'a>) -> Self {
        BorrowedFieldsAbi::to_ffi(value)
    }
}

impl<'a> From<BorrowedFieldsAbi<'a>> for BorrowedFields<'a>{
    fn from(value: BorrowedFieldsAbi<'a>) -> Self {
        BorrowedFieldsAbi::from_ffi(value)
    }
}

impl<'a> BorrowedFields<'a> {
    pub fn from_bar_and_strings<'x>(bar : &'x Bar<'x, 'x>, dstr16 : &'x [u16], utf8_str : &'x str) -> BorrowedFields<'x> {
        let ret = unsafe { BorrowedFields_from_bar_and_strings(bar, dstr16.into(), utf8_str.as_bytes().into()) };
        
        ret.from_ffi()
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn BorrowedFields_from_bar_and_strings<'x>(bar : &'x Bar<'x, 'x>, dstr16 : diplomat_runtime::DiplomatStr16Slice<'x>, utf8_str : diplomat_runtime::DiplomatStrSlice<'x>) -> BorrowedFieldsAbi<'x>;
}