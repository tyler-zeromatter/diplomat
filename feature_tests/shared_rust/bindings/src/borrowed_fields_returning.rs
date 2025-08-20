#[repr(C)]
pub struct BorrowedFieldsReturning {
    pub bytes: &[TODO],
}

impl BorrowedFieldsReturning {
}

#[link(name = "somelib")]
unsafe extern "C" {
}