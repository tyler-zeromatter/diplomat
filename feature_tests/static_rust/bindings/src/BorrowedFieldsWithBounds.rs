#[repr(C)]
pub struct BorrowedFieldsWithBounds {

}

impl BorrowedFieldsWithBounds {
    fn from_foo_and_strings(foo : TODO(), dstr16_x : TODO(), utf8_str_z : TODO()) {
        unsafe { BorrowedFieldsWithBounds_from_foo_and_strings(foo, dstr16_x, utf8_str_z) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn BorrowedFieldsWithBounds_from_foo_and_strings(foo : TODO(), dstr16_x : TODO(), utf8_str_z : TODO());

}