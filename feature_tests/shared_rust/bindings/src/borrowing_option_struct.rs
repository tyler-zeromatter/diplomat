pub struct BorrowingOptionStruct<'a> {
    pub a: &'a Option<[u8]>,
}

#[repr(C)]
pub(crate) struct BorrowingOptionStructAbi<'a> {
    a : &'a diplomat_runtime::DiplomatOption<diplomat_runtime::DiplomatStrSlice>,
}

impl<'a> BorrowingOptionStructAbi<'a> {
    pub(crate) fn from_ffi(self) -> BorrowingOptionStruct<'a> {
        BorrowingOptionStruct {
            
            a: self.a.into_converted_option(),
            
        }
    }

    pub(crate) fn to_ffi(this : BorrowingOptionStruct<'a>) -> BorrowingOptionStructAbi<'a> {
        BorrowingOptionStructAbi {
            
            a : this.a.map(|ok| { ok.into() }).into(),
            
        }
    }
}

impl<'a> From<BorrowingOptionStruct<'a>> for BorrowingOptionStructAbi<'a>{
    fn from(value: BorrowingOptionStruct<'a>) -> Self {
        BorrowingOptionStructAbi::to_ffi(value)
    }
}

impl<'a> From<BorrowingOptionStructAbi<'a>> for BorrowingOptionStruct<'a>{
    fn from(value: BorrowingOptionStructAbi<'a>) -> Self {
        BorrowingOptionStructAbi::from_ffi(value)
    }
}

impl<'a> BorrowingOptionStruct<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}