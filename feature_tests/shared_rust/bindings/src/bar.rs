use super::Foo;
pub struct Bar;

impl Bar {
    pub fn foo(&self) -> &Foo {
            // TODO: writeable conversions.
        unsafe { Bar_foo(self) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn Bar_foo(this: &Bar) -> &Foo;

}