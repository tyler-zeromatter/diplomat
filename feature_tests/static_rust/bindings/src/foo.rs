use super::Bar;
use super::BorrowedFields;
use super::BorrowedFieldsReturning;
use super::BorrowedFieldsWithBounds;
pub struct Foo;

impl Foo {
    fn new(x : &[TODO]) -> Box<Foo> {
            // TODO: writeable conversions.
        unsafe { Foo_new(x) }
    }

    fn get_bar(&self) -> Box<Bar> {
            // TODO: writeable conversions.
        unsafe { Foo_get_bar(self) }
    }

    fn as_returning(&self) -> BorrowedFieldsReturning {
            // TODO: writeable conversions.
        unsafe { Foo_as_returning(self) }
    }

    fn extract_from_fields(fields : BorrowedFields) -> Box<Foo> {
            // TODO: writeable conversions.
        unsafe { Foo_extract_from_fields(fields) }
    }

    fn extract_from_bounds(bounds : BorrowedFieldsWithBounds, another_string : &[TODO]) -> Box<Foo> {
            // TODO: writeable conversions.
        unsafe { Foo_extract_from_bounds(bounds, another_string) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Foo_new(x : &[TODO]) -> Box<Foo>;

    fn Foo_get_bar(this: &Foo) -> Box<Bar>;

    fn Foo_as_returning(this: &Foo) -> BorrowedFieldsReturning;

    fn Foo_extract_from_fields(fields : BorrowedFields) -> Box<Foo>;

    fn Foo_extract_from_bounds(bounds : BorrowedFieldsWithBounds, another_string : &[TODO]) -> Box<Foo>;

}