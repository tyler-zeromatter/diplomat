#[repr(C)]
pub struct BorrowingOptionStruct<'a> {
    pub a: &'a diplomat_runtime::DiplomatOption::<diplomat_runtime::DiplomatStrSlice::<'a>>,
}

impl<'a> BorrowingOptionStruct<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}