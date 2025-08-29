use super::Foo;
pub struct BorrowedFieldsWithBounds<'a, 'b, 'c> {
    pub field_a: &'a [u16],
    pub field_b: &'b [u8],
    pub field_c: &'c str,
}

#[repr(C)]
pub(crate) struct BorrowedFieldsWithBoundsAbi<'a, 'b, 'c> {
    field_a : diplomat_runtime::DiplomatStr16Slice<'a>,
    field_b : diplomat_runtime::DiplomatStrSlice<'b>,
    field_c : diplomat_runtime::DiplomatStrSlice<'c>,
}

impl<'a, 'b, 'c> BorrowedFieldsWithBoundsAbi<'a, 'b, 'c> {
    pub(crate) fn from_ffi(self) -> BorrowedFieldsWithBounds<'a, 'b, 'c>{
        BorrowedFieldsWithBounds {
            
            field_a: self.field_a.into(),
            
            field_b: self.field_b.into(),
            
            field_c: unsafe { str::from_utf8_unchecked(self.field_c.into()).into()},
            
        }
    }

    pub (crate) fn to_ffi(this : BorrowedFieldsWithBounds<'a, 'b, 'c>) -> BorrowedFieldsWithBoundsAbi<'a, 'b, 'c>{
        BorrowedFieldsWithBoundsAbi {
            
            field_a : this.field_a.into(),
            
            field_b : this.field_b.into(),
            
            field_c : this.field_c.as_bytes().into(),
            
        }
    }
}

impl<'a, 'b, 'c> From<BorrowedFieldsWithBounds<'a, 'b, 'c>> for BorrowedFieldsWithBoundsAbi<'a, 'b, 'c>{
    fn from(value: BorrowedFieldsWithBounds<'a, 'b, 'c>) -> Self {
        BorrowedFieldsWithBoundsAbi::to_ffi(value)
    }
}

impl<'a, 'b, 'c> From<BorrowedFieldsWithBoundsAbi<'a, 'b, 'c>> for BorrowedFieldsWithBounds<'a, 'b, 'c>{
    fn from(value: BorrowedFieldsWithBoundsAbi<'a, 'b, 'c>) -> Self {
        value.from_ffi()
    }
}

impl<'a, 'b, 'c> BorrowedFieldsWithBounds<'a, 'b, 'c> {
    pub fn from_foo_and_strings<'x, 'y: 'x, 'z: 'y + 'x>(foo : &'x Foo<'y>, dstr16_x : &'x [u16], utf8_str_z : &'z str) -> BorrowedFieldsWithBounds<'x, 'y, 'z> {
        let ret = unsafe { BorrowedFieldsWithBounds_from_foo_and_strings(foo, dstr16_x.into(), utf8_str_z.as_bytes().into()) };
        
        ret.from_ffi()
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn BorrowedFieldsWithBounds_from_foo_and_strings<'x, 'y: 'x, 'z: 'y + 'x>(foo : &'x Foo<'y>, dstr16_x : diplomat_runtime::DiplomatStr16Slice<'x>, utf8_str_z : diplomat_runtime::DiplomatStrSlice<'z>) -> BorrowedFieldsWithBoundsAbi<'x, 'y, 'z>;
}