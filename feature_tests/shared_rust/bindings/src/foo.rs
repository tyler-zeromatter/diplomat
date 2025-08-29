use super::Bar;
use super::BorrowedFields;
use super::BorrowedFieldsReturning;
use super::BorrowedFieldsWithBounds;
use super::borrowed_fields::BorrowedFieldsAbi;
use super::borrowed_fields_returning::BorrowedFieldsReturningAbi;
use super::borrowed_fields_with_bounds::BorrowedFieldsWithBoundsAbi;
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
    pub fn new(x : &'a [u8]) -> Box<Foo<'a>> {
        let ret = unsafe { Foo_new(x.into()) };
        
        ret
    
    }

    pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> {
        let ret = unsafe { Foo_get_bar(self) };
        
        ret
    
    }

    pub fn as_returning<'anon_0>(&'anon_0 self) -> BorrowedFieldsReturning<'a> {
        let ret = unsafe { Foo_as_returning(self) };
        
        ret.from_ffi()
    
    }

    pub fn extract_from_fields(fields : BorrowedFields<'a>) -> Box<Foo<'a>> {
        let ret = unsafe { Foo_extract_from_fields(fields.into()) };
        
        ret
    
    }

    pub fn extract_from_bounds<'x, 'y: 'a + 'x, 'z: 'y + 'a + 'x>(bounds : BorrowedFieldsWithBounds<'x, 'y, 'z>, another_string : &'a [u8]) -> Box<Foo<'a>> {
        let ret = unsafe { Foo_extract_from_bounds(bounds.into(), another_string.into()) };
        
        ret
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Foo_new<'a>(x : diplomat_runtime::DiplomatStrSlice<'a>) -> Box<Foo<'a>>;

    fn Foo_get_bar<'a: 'b, 'b>(this: &'b Foo) -> Box<Bar<'b, 'a>>;

    fn Foo_as_returning<'a, 'anon_0>(this: &'anon_0 Foo) -> BorrowedFieldsReturningAbi<'a>;

    fn Foo_extract_from_fields<'a>(fields : BorrowedFieldsAbi<'a>) -> Box<Foo<'a>>;

    fn Foo_extract_from_bounds<'a, 'x, 'y: 'a + 'x, 'z: 'y + 'a + 'x>(bounds : BorrowedFieldsWithBoundsAbi<'x, 'y, 'z>, another_string : diplomat_runtime::DiplomatStrSlice<'a>) -> Box<Foo<'a>>;

    fn Foo_destroy(this : *mut Foo);
}