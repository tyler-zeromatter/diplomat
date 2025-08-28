use super::Bar;
use super::BorrowedFields;
use super::BorrowedFieldsReturning;
use super::BorrowedFieldsWithBounds;
use std::marker::PhantomData;

pub struct Foo<'a> {
    a_marker : PhantomData<&'a ()>,
}

impl<'a> Drop for Foo<'a> {
    fn drop(&mut self) {
        unsafe { Foo_destroy(self) }
    }
}

impl<'a> Foo<'a> {
    pub fn new<'a>(x : &'a [u8]) -> Box<Foo<'a>> {
        let ret = unsafe { Foo_new(&x.into()) };
        
        ret
    }

    pub fn get_bar<'a, 'b>(&'b self) -> Box<Bar<'b, 'a>> {
        let ret = unsafe { Foo_get_bar(self) };
        
        ret
    }

    pub fn as_returning<'a, 'anon_0>(&'anon_0 self) -> BorrowedFieldsReturning<'a> {
        let ret = unsafe { Foo_as_returning(self) };
        
        ret
    }

    pub fn extract_from_fields<'a>(fields : BorrowedFields<'a>) -> Box<Foo<'a>> {
        let ret = unsafe { Foo_extract_from_fields(fields) };
        
        ret
    }

    pub fn extract_from_bounds<'a, 'x, 'y, 'z>(bounds : BorrowedFieldsWithBounds<'x, 'y, 'z>, another_string : &'a [u8]) -> Box<Foo<'a>> {
        let ret = unsafe { Foo_extract_from_bounds(bounds, &another_string.into()) };
        
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Foo_new<'a>(x : &'a diplomat_runtime::DiplomatStrSlice) -> Box<Foo<'a>>;

    fn Foo_get_bar<'a, 'b>(this: &'b Foo) -> Box<Bar<'b, 'a>>;

    fn Foo_as_returning<'a, 'anon_0>(this: &'anon_0 Foo) -> BorrowedFieldsReturning<'a>;

    fn Foo_extract_from_fields<'a>(fields : BorrowedFields<'a>) -> Box<Foo<'a>>;

    fn Foo_extract_from_bounds<'a, 'x, 'y, 'z>(bounds : BorrowedFieldsWithBounds<'x, 'y, 'z>, another_string : &'a diplomat_runtime::DiplomatStrSlice) -> Box<Foo<'a>>;

    fn Foo_destroy(this : *mut Foo);
}