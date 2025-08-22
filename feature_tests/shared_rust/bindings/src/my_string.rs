pub struct MyString;

impl MyString {
    pub fn new(v : &[TODO]) -> Box<MyString> {
        let ret = unsafe { MyString_new(v) };
        ret
    }

    pub fn new_unsafe(v : &[TODO]) -> Box<MyString> {
        let ret = unsafe { MyString_new_unsafe(v) };
        ret
    }

    pub fn new_owned(v : &[TODO]) -> Box<MyString> {
        let ret = unsafe { MyString_new_owned(v) };
        ret
    }

    pub fn new_from_first(v : &[TODO]) -> Box<MyString> {
        let ret = unsafe { MyString_new_from_first(v) };
        ret
    }

    pub fn set_str(&mut self, new_str : &[TODO]) {
        let ret = unsafe { MyString_set_str(self, new_str) };
        ret
    }

    pub fn get_str(&self) -> String {
        let write = unsafe {
            diplomat_runtime::diplomat_buffer_write_create(0)
        };
        let ret = unsafe { MyString_get_str(self, write) };
        let out_str = unsafe {
            let write_ref = write.as_ref().unwrap();
            let buf = diplomat_runtime::diplomat_buffer_write_get_bytes(write_ref);
            let len = diplomat_runtime::diplomat_buffer_write_len(write_ref);
    
            if !buf.is_null() {
                String::from_raw_parts(buf, len, len)
            } else {
                panic!("Could not read buffer, growth failed.")
            }
        };
    
        unsafe {
            diplomat_runtime::diplomat_buffer_write_destroy(write);
        }
        out_str
    }

    pub fn get_static_str() -> &[TODO] {
        let ret = unsafe { MyString_get_static_str() };
        ret
    }

    pub fn string_transform(foo : &[TODO]) -> String {
        let write = unsafe {
            diplomat_runtime::diplomat_buffer_write_create(0)
        };
        let ret = unsafe { MyString_string_transform(foo, write) };
        let out_str = unsafe {
            let write_ref = write.as_ref().unwrap();
            let buf = diplomat_runtime::diplomat_buffer_write_get_bytes(write_ref);
            let len = diplomat_runtime::diplomat_buffer_write_len(write_ref);
    
            if !buf.is_null() {
                String::from_raw_parts(buf, len, len)
            } else {
                panic!("Could not read buffer, growth failed.")
            }
        };
    
        unsafe {
            diplomat_runtime::diplomat_buffer_write_destroy(write);
        }
        out_str
    }

    pub fn borrow(&self) -> &[TODO] {
        let ret = unsafe { MyString_borrow(self) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyString_new(v : &[TODO]) -> Box<MyString>;

    fn MyString_new_unsafe(v : &[TODO]) -> Box<MyString>;

    fn MyString_new_owned(v : &[TODO]) -> Box<MyString>;

    fn MyString_new_from_first(v : &[TODO]) -> Box<MyString>;

    fn MyString_set_str(this: &mut MyString, new_str : &[TODO]);

    fn MyString_get_str(this: &MyString, write : &mut diplomat_runtime::DiplomatWrite) -> String;

    fn MyString_get_static_str() -> &[TODO];

    fn MyString_string_transform(foo : &[TODO], write : &mut diplomat_runtime::DiplomatWrite) -> String;

    fn MyString_borrow(this: &MyString) -> &[TODO];

}