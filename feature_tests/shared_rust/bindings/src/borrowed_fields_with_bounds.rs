use super::Foo;
#[repr(C)]
pub struct BorrowedFieldsWithBounds<'a, 'b, 'c> {
    pub field_a: &'a [u16],
    pub field_b: &'b [u8],
    pub field_c: &'c str,
}

impl<'a, 'b, 'c> BorrowedFieldsWithBounds<'a, 'b, 'c> {
    pub fn from_foo_and_strings<'x, 'y: 'x, 'z: 'y + 'x>(foo : &'x Foo<'y>, dstr16_x : &'x [u16], utf8_str_z : &'z str) -> BorrowedFieldsWithBounds<'x, 'y, 'z> {
        let ret = unsafe { BorrowedFieldsWithBounds_from_foo_and_strings(foo, dstr16_x.into(), utf8_str_z.as_bytes().into()) };
        
        ret
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn BorrowedFieldsWithBounds_from_foo_and_strings<'x, 'y: 'x, 'z: 'y + 'x>(foo : &'x Foo<'y>, dstr16_x : diplomat_runtime::DiplomatStr16Slice<'x>, utf8_str_z : diplomat_runtime::DiplomatStrSlice<'z>) -> BorrowedFieldsWithBounds<'x, 'y, 'z>;
}