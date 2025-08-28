use super::Bar;
use super::BorrowedFields;
use super::BorrowedFieldsWithBounds;
use super::Foo;
#[repr(C)]
pub struct NestedBorrowedFields<'x, 'y, 'z> {
    pub fields: BorrowedFields<'x>,
    pub bounds: BorrowedFieldsWithBounds<'x, 'y, 'y>,
    pub bounds2: BorrowedFieldsWithBounds<'z, 'z, 'z>,
}

impl<'x, 'y, 'z> NestedBorrowedFields<'x, 'y, 'z> {
    pub fn from_bar_and_foo_and_strings<'x, 'y: 'x, 'z>(bar : &'x Bar<'x, 'y>, foo : &'z Foo<'z>, dstr16_x : &'x [u16], dstr16_z : &'z [u16], utf8_str_y : &'y str, utf8_str_z : &'z str) -> NestedBorrowedFields<'x, 'y, 'z> {
        let ret = unsafe { NestedBorrowedFields_from_bar_and_foo_and_strings(bar, foo, dstr16_x.into(), dstr16_z.into(), utf8_str_y.as_bytes().into(), utf8_str_z.as_bytes().into()) };
        
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn NestedBorrowedFields_from_bar_and_foo_and_strings<'x, 'y: 'x, 'z>(bar : &'x Bar<'x, 'y>, foo : &'z Foo<'z>, dstr16_x : diplomat_runtime::DiplomatStr16Slice<'x>, dstr16_z : diplomat_runtime::DiplomatStr16Slice<'z>, utf8_str_y : diplomat_runtime::DiplomatStrSlice<'y>, utf8_str_z : diplomat_runtime::DiplomatStrSlice<'z>) -> NestedBorrowedFields<'x, 'y, 'z>;
}