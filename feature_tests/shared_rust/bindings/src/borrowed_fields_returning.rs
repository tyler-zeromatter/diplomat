#[repr(C)]
pub struct BorrowedFieldsReturning {
    pub bytes: &[u8],
}

impl BorrowedFieldsReturning {
}

#[link(name = "somelib")]
unsafe extern "C" {}