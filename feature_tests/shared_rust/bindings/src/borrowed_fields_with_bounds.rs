use super::Foo;
#[repr(C)]
pub struct BorrowedFieldsWithBounds {
    pub field_a: &[u16],
    pub field_b: &[u8],
    pub field_c: &String,
}

impl BorrowedFieldsWithBounds {
    pub fn from_foo_and_strings(foo : &Foo, dstr16_x : &[u16], utf8_str_z : &String) -> BorrowedFieldsWithBounds {
        let ret = unsafe { BorrowedFieldsWithBounds_from_foo_and_strings(foo, dstr16_x.into(), utf8_str_z.into()) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn BorrowedFieldsWithBounds_from_foo_and_strings(foo : &Foo, dstr16_x : diplomat_runtime::DiplomatStrSlice, utf8_str_z : diplomat_runtime::DiplomatStrSlice) -> BorrowedFieldsWithBounds;

}