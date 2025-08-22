use super::Bar;
use super::BorrowedFields;
use super::BorrowedFieldsWithBounds;
use super::Foo;
#[repr(C)]
pub struct NestedBorrowedFields {
    pub fields: BorrowedFields,
    pub bounds: BorrowedFieldsWithBounds,
    pub bounds2: BorrowedFieldsWithBounds,
}

impl NestedBorrowedFields {
    pub fn from_bar_and_foo_and_strings(bar : &Bar, foo : &Foo, dstr16_x : &[TODO], dstr16_z : &[TODO], utf8_str_y : &[TODO], utf8_str_z : &[TODO]) -> NestedBorrowedFields {
        let ret = unsafe { NestedBorrowedFields_from_bar_and_foo_and_strings(bar, foo, dstr16_x, dstr16_z, utf8_str_y, utf8_str_z) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn NestedBorrowedFields_from_bar_and_foo_and_strings(bar : &Bar, foo : &Foo, dstr16_x : &[TODO], dstr16_z : &[TODO], utf8_str_y : &[TODO], utf8_str_z : &[TODO]) -> NestedBorrowedFields;

}