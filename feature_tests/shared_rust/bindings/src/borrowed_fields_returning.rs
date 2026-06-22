#[repr(C)]
pub struct BorrowedFieldsReturning<'a> {
    pub bytes: diplomat_runtime::DiplomatStrSlice::<'a>,
}

impl<'a> BorrowedFieldsReturning<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}