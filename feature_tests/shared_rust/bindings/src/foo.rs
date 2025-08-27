use super::Bar;
use super::BorrowedFields;
use super::BorrowedFieldsReturning;
use super::BorrowedFieldsWithBounds;
pub struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        unsafe { Foo_destroy(self) }
    }
}

impl Foo {
    pub fn new(x : &'a [u8]) -> Box<Foo><'a> {
        let ret = unsafe { Foo_new(x.into()) };
        ret
    }

    pub fn get_bar(&self) -> Box<Bar><'b, 'a> {
        let ret = unsafe { Foo_get_bar(self) };
        ret
    }

    pub fn as_returning(&self) -> BorrowedFieldsReturning<'a> {
        let ret = unsafe { Foo_as_returning(self) };
        ret
    }

    pub fn extract_from_fields(fields : BorrowedFields<'a>) -> Box<Foo><'a> {
        let ret = unsafe { Foo_extract_from_fields(fields) };
        ret
    }

    pub fn extract_from_bounds(bounds : BorrowedFieldsWithBounds<'x, 'y, 'z>, another_string : &'a [u8]) -> Box<Foo><'a> {
        let ret = unsafe { Foo_extract_from_bounds(bounds, another_string.into()) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Foo_new(x : &'a diplomat_runtime::DiplomatStrSlice) -> Box<Foo><'a>;

    fn Foo_get_bar(this: &Foo) -> Box<Bar><'b, 'a>;

    fn Foo_as_returning(this: &Foo) -> BorrowedFieldsReturning<'a>;

    fn Foo_extract_from_fields(fields : BorrowedFields<'a>) -> Box<Foo><'a>;

    fn Foo_extract_from_bounds(bounds : BorrowedFieldsWithBounds<'x, 'y, 'z>, another_string : &'a diplomat_runtime::DiplomatStrSlice) -> Box<Foo><'a>;

    fn Foo_destroy(this : *mut Foo);
}