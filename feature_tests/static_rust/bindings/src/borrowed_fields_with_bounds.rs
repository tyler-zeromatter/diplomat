use super::Foo;
#[repr(C)]
pub struct BorrowedFieldsWithBounds {

}

impl BorrowedFieldsWithBounds {
    pub fn from_foo_and_strings(foo : &Foo, dstr16_x : &[TODO], utf8_str_z : &[TODO]) -> BorrowedFieldsWithBounds {
            // TODO: writeable conversions.
        unsafe { BorrowedFieldsWithBounds_from_foo_and_strings(foo, dstr16_x, utf8_str_z) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn BorrowedFieldsWithBounds_from_foo_and_strings(foo : &Foo, dstr16_x : &[TODO], utf8_str_z : &[TODO]) -> BorrowedFieldsWithBounds;

}