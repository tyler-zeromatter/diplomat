pub struct MyString;

impl MyString {
    pub fn new(v : &[TODO]) -> Box<MyString> {
            // TODO: writeable conversions.
        unsafe { MyString_new(v) }
    }

    pub fn new_unsafe(v : &[TODO]) -> Box<MyString> {
            // TODO: writeable conversions.
        unsafe { MyString_new_unsafe(v) }
    }

    pub fn new_owned(v : &[TODO]) -> Box<MyString> {
            // TODO: writeable conversions.
        unsafe { MyString_new_owned(v) }
    }

    pub fn new_from_first(v : &[TODO]) -> Box<MyString> {
            // TODO: writeable conversions.
        unsafe { MyString_new_from_first(v) }
    }

    pub fn set_str(&mut self, new_str : &[TODO]) {
            // TODO: writeable conversions.
        unsafe { MyString_set_str(self, new_str) }
    }

    pub fn get_str(&self) {
            // TODO: writeable conversions.
        unsafe { MyString_get_str(self, output) }
    }

    pub fn get_static_str() -> &[TODO] {
            // TODO: writeable conversions.
        unsafe { MyString_get_static_str() }
    }

    pub fn string_transform(foo : &[TODO]) {
            // TODO: writeable conversions.
        unsafe { MyString_string_transform(foo, output) }
    }

    pub fn borrow(&self) -> &[TODO] {
            // TODO: writeable conversions.
        unsafe { MyString_borrow(self) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyString_new(v : &[TODO]) -> Box<MyString>;

    fn MyString_new_unsafe(v : &[TODO]) -> Box<MyString>;

    fn MyString_new_owned(v : &[TODO]) -> Box<MyString>;

    fn MyString_new_from_first(v : &[TODO]) -> Box<MyString>;

    fn MyString_set_str(this: &mut MyString, new_str : &[TODO]);

    fn MyString_get_str(this: &MyString, output : &mut DiplomatWrite);

    fn MyString_get_static_str() -> &[TODO];

    fn MyString_string_transform(foo : &[TODO], output : &mut DiplomatWrite);

    fn MyString_borrow(this: &MyString) -> &[TODO];

}