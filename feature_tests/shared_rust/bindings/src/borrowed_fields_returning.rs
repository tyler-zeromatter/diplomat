pub struct BorrowedFieldsReturning<'a> {
    pub bytes: &'a [u8],
}

#[repr(C)]
pub(crate) struct BorrowedFieldsReturningAbi<'a> {
    
    bytes : diplomat_runtime::DiplomatStrSlice<'a>,
    
}

impl<'a> BorrowedFieldsReturningAbi<'a> {
    fn from_ffi(self) -> BorrowedFieldsReturning<'a>{
        BorrowedFieldsReturning {
            
                bytes: self.bytes.into(),
            
        }
    }
}

impl<'a> BorrowedFieldsReturning<'a> {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}