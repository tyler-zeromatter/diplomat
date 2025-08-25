use super::Foo;
pub struct Bar;

impl Drop for Bar {
    fn drop(&mut self) {
        unsafe { Bar_destroy(self) }
    }
}

impl Bar {
    pub fn foo(&self) -> &Foo {
        let ret = unsafe { Bar_foo(self) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn Bar_foo(this: &Bar) -> &Foo;

    fn Bar_destroy(this : *mut Bar);
}