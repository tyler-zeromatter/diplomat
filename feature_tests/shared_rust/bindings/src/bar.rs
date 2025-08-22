use super::Foo;
pub struct Bar;

impl Bar {
    pub fn foo(&self) -> &Foo {
        let ret = unsafe { Bar_foo(self) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Bar_foo(this: &Bar) -> &Foo;

}