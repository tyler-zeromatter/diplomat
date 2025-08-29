pub struct BorrowedFieldsReturning<'a> {
    pub bytes: &'a [u8],
}

#[repr(C)]
pub(crate) struct BorrowedFieldsReturningAbi<'a> {
    bytes : diplomat_runtime::DiplomatStrSlice<'a>,
}

impl<'a> BorrowedFieldsReturningAbi<'a> {
    pub(crate) fn from_ffi(self) -> BorrowedFieldsReturning<'a> {
        BorrowedFieldsReturning {
            
            bytes: self.bytes.into(),
            
        }
    }

    pub(crate) fn to_ffi(this : BorrowedFieldsReturning<'a>) -> BorrowedFieldsReturningAbi<'a> {
        BorrowedFieldsReturningAbi {
            
            bytes : this.bytes.into(),
            
        }
    }
}

impl<'a> From<BorrowedFieldsReturning<'a>> for BorrowedFieldsReturningAbi<'a>{
    fn from(value: BorrowedFieldsReturning<'a>) -> Self {
        BorrowedFieldsReturningAbi::to_ffi(value)
    }
}

impl<'a> From<BorrowedFieldsReturningAbi<'a>> for BorrowedFieldsReturning<'a>{
    fn from(value: BorrowedFieldsReturningAbi<'a>) -> Self {
        BorrowedFieldsReturningAbi::from_ffi(value)
    }
}

impl<'a> BorrowedFieldsReturning<'a> {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}