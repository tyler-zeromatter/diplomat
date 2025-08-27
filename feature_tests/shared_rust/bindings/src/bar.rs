use super::Foo;
pub struct Bar;

impl Drop for Bar {
    fn drop(&mut self) {
        unsafe { Bar_destroy(self) }
    }
}

impl Bar {
    pub fn foo(&self) -> &'b Foo<'a> {
        let ret = unsafe { Bar_foo(self) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Bar_foo(this: &Bar) -> &'b Foo<'a>;

    fn Bar_destroy(this : *mut Bar);
}