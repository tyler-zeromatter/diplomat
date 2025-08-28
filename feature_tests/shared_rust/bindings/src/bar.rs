use super::Foo;
use std::marker::PhantomData;

pub struct Bar<'b, 'a> {
    b_marker : PhantomData<&'b ()>,
    a_marker : PhantomData<&'a ()>,
}

impl<'b, 'a> Drop for Bar<'b, 'a> {
    fn drop(&mut self) {
        unsafe { Bar_destroy(self) }
    }
}

impl<'b, 'a> Bar<'b, 'a> {
    pub fn foo<'b, 'a: 'b>(&'b self) -> &'b Foo<'a> {
        let ret = unsafe { Bar_foo(self) };
        
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Bar_foo<'b, 'a: 'b>(this: &'b Bar) -> &'b Foo<'a>;

    fn Bar_destroy(this : *mut Bar);
}