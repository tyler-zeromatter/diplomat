use super::Foo;
#[repr(C)]
pub struct BorrowedFieldsWithBounds {
    pub field_a: &[TODO],
    pub field_b: &[TODO],
    pub field_c: &[TODO],
}

impl BorrowedFieldsWithBounds {
    pub fn from_foo_and_strings(foo : &Foo, dstr16_x : &[TODO], utf8_str_z : &[TODO]) -> BorrowedFieldsWithBounds {
        let ret = unsafe { BorrowedFieldsWithBounds_from_foo_and_strings(foo, dstr16_x, utf8_str_z) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn BorrowedFieldsWithBounds_from_foo_and_strings(foo : &Foo, dstr16_x : &[TODO], utf8_str_z : &[TODO]) -> BorrowedFieldsWithBounds;

}