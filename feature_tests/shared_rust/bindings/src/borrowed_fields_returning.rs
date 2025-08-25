#[repr(C)]
pub struct BorrowedFieldsReturning {
    pub bytes: &[u8],
}

impl BorrowedFieldsReturning {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}