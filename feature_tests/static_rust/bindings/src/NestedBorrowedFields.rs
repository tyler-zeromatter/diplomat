use super::Bar;
use super::Foo;
#[repr(C)]
pub struct NestedBorrowedFields {

}

impl NestedBorrowedFields {
    fn from_bar_and_foo_and_strings(bar : Bar, foo : Foo, dstr16_x : TODO(), dstr16_z : TODO(), utf8_str_y : TODO(), utf8_str_z : TODO()) -> NestedBorrowedFields {
        unsafe { NestedBorrowedFields_from_bar_and_foo_and_strings(bar, foo, dstr16_x, dstr16_z, utf8_str_y, utf8_str_z) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn NestedBorrowedFields_from_bar_and_foo_and_strings(bar : Bar, foo : Foo, dstr16_x : TODO(), dstr16_z : TODO(), utf8_str_y : TODO(), utf8_str_z : TODO()) -> NestedBorrowedFields;

}