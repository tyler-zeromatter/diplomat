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
    pub fn from_bar_and_foo_and_strings(bar : Bar, foo : Foo, dstr16_x : [u16], dstr16_z : [u16], utf8_str_y : String, utf8_str_z : String) -> NestedBorrowedFields {
        let ret = unsafe { NestedBorrowedFields_from_bar_and_foo_and_strings(bar, foo, dstr16_x.into(), dstr16_z.into(), utf8_str_y.into(), utf8_str_z.into()) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn NestedBorrowedFields_from_bar_and_foo_and_strings(bar : Bar, foo : Foo, dstr16_x : diplomat_runtime::DiplomatStrSlice, dstr16_z : diplomat_runtime::DiplomatStrSlice, utf8_str_y : diplomat_runtime::DiplomatStrSlice, utf8_str_z : diplomat_runtime::DiplomatStrSlice) -> NestedBorrowedFields;
}