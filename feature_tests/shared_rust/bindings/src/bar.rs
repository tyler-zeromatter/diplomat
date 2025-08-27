use super::Foo;
pub struct Bar<'b, 'a>;

impl Drop for Bar {
    fn drop(&mut self) {
        unsafe { Bar_destroy(self) }
    }
}

impl<'b, 'a> Bar<'b, 'a> {
    pub fn foo<'b, 'a>(&self) -> &'b Foo<'a> {
        let ret = unsafe { Bar_foo(self) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Bar_foo<'b, 'a>(this: &Bar) -> &'b Foo<'a>;

    fn Bar_destroy(this : *mut Bar);
}