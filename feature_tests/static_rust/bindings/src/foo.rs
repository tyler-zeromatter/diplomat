use super::Bar;
use super::BorrowedFields;
use super::BorrowedFieldsReturning;
use super::BorrowedFieldsWithBounds;
pub struct Foo;

impl Foo {
    fn new(x : &[TODO]) -> Foo {
            // TODO: writeable conversions.
        unsafe { Foo_new(x) }
    }

    fn get_bar(&self) -> Bar {
            // TODO: writeable conversions.
        unsafe { Foo_get_bar(self) }
    }

    fn as_returning(&self) -> BorrowedFieldsReturning {
            // TODO: writeable conversions.
        unsafe { Foo_as_returning(self) }
    }

    fn extract_from_fields(fields : BorrowedFields) -> Foo {
            // TODO: writeable conversions.
        unsafe { Foo_extract_from_fields(fields) }
    }

    fn extract_from_bounds(bounds : BorrowedFieldsWithBounds, another_string : &[TODO]) -> Foo {
            // TODO: writeable conversions.
        unsafe { Foo_extract_from_bounds(bounds, another_string) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Foo_new(x : &[TODO]) -> Foo;

    fn Foo_get_bar(this: &Foo) -> Bar;

    fn Foo_as_returning(this: &Foo) -> BorrowedFieldsReturning;

    fn Foo_extract_from_fields(fields : BorrowedFields) -> Foo;

    fn Foo_extract_from_bounds(bounds : BorrowedFieldsWithBounds, another_string : &[TODO]) -> Foo;

}