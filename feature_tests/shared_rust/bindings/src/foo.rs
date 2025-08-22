use super::Bar;
use super::BorrowedFields;
use super::BorrowedFieldsReturning;
use super::BorrowedFieldsWithBounds;
pub struct Foo;

impl Foo {
    pub fn new(x : &[TODO]) -> Box<Foo> {
        let ret = unsafe { Foo_new(x) };
        ret
    }

    pub fn get_bar(&self) -> Box<Bar> {
        let ret = unsafe { Foo_get_bar(self) };
        ret
    }

    pub fn as_returning(&self) -> BorrowedFieldsReturning {
        let ret = unsafe { Foo_as_returning(self) };
        ret
    }

    pub fn extract_from_fields(fields : BorrowedFields) -> Box<Foo> {
        let ret = unsafe { Foo_extract_from_fields(fields) };
        ret
    }

    pub fn extract_from_bounds(bounds : BorrowedFieldsWithBounds, another_string : &[TODO]) -> Box<Foo> {
        let ret = unsafe { Foo_extract_from_bounds(bounds, another_string) };
        ret
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