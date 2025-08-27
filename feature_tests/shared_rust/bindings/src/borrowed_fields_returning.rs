#[repr(C)]
pub struct BorrowedFieldsReturning<'a> {
    pub bytes: &'a [u8],
}

impl<'a> BorrowedFieldsReturning<'a> {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}