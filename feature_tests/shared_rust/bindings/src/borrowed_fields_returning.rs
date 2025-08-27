#[repr(C)]
pub struct BorrowedFieldsReturning<'a> {
    pub bytes: [u8]<'a>,
}

impl BorrowedFieldsReturning {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}