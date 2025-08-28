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
    pub fn from_bar_and_foo_and_strings<'x, 'y, 'z>(bar : &'x Bar<'x, 'y>, foo : &'z Foo<'z>, dstr16_x : &'x [u16], dstr16_z : &'z [u16], utf8_str_y : &'y String, utf8_str_z : &'z String) -> NestedBorrowedFields<'x, 'y, 'z> {
        let ret = unsafe { NestedBorrowedFields_from_bar_and_foo_and_strings(bar, foo, &dstr16_x.into(), &dstr16_z.into(), &utf8_str_y.into(), &utf8_str_z.into()) };
        
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn NestedBorrowedFields_from_bar_and_foo_and_strings<'x, 'y, 'z>(bar : &'x Bar<'x, 'y>, foo : &'z Foo<'z>, dstr16_x : &'x diplomat_runtime::DiplomatStrSlice, dstr16_z : &'z diplomat_runtime::DiplomatStrSlice, utf8_str_y : &'y diplomat_runtime::DiplomatStrSlice, utf8_str_z : &'z diplomat_runtime::DiplomatStrSlice) -> NestedBorrowedFields<'x, 'y, 'z>;
}