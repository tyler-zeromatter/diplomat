pub struct MyString;

impl MyString {
    pub fn new(v : &[u8]) -> Box<MyString> {
        let ret = unsafe { MyString_new(v.into()) };
        ret
    }

    pub fn new_unsafe(v : &String) -> Box<MyString> {
        let ret = unsafe { MyString_new_unsafe(v.into()) };
        ret
    }

    pub fn new_owned(v : [u8]) -> Box<MyString> {
        let ret = unsafe { MyString_new_owned(v.into()) };
        ret
    }

    pub fn new_from_first(v : &[TODO]) -> Box<MyString> {
        let ret = unsafe { MyString_new_from_first(v) };
        ret
    }

    pub fn set_str(&mut self, new_str : &[u8]) {
        let ret = unsafe { MyString_set_str(self, new_str.into()) };
        ret
    }

    pub fn get_str(&self) -> String {
        let write = unsafe {
            diplomat_runtime::diplomat_buffer_write_create(0)
        };
        let ret = unsafe { MyString_get_str(self, write.as_mut().unwrap()) };
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

    pub fn get_static_str() -> &'staticString {
        let ret = unsafe { MyString_get_static_str() };
        ret
    }

    pub fn string_transform(foo : &String) -> String {
        let write = unsafe {
            diplomat_runtime::diplomat_buffer_write_create(0)
        };
        let ret = unsafe { MyString_string_transform(foo.into(), write.as_mut().unwrap()) };
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

    pub fn borrow(&self) -> &[u8] {
        let ret = unsafe { MyString_borrow(self) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyString_new(v : diplomat_runtime::DiplomatStrSlice) -> Box<MyString>;

    fn MyString_new_unsafe(v : diplomat_runtime::DiplomatStrSlice) -> Box<MyString>;

    fn MyString_new_owned(v : diplomat_runtime::DiplomatStrSlice) -> Box<MyString>;

    fn MyString_new_from_first(v : &[TODO]) -> Box<MyString>;

    fn MyString_set_str(this: &mut MyString, new_str : diplomat_runtime::DiplomatStrSlice);

    fn MyString_get_str(this: &MyString, write : &mut diplomat_runtime::DiplomatWrite) -> ();

    fn MyString_get_static_str() -> diplomat_runtime::DiplomatStrSlice;

    fn MyString_string_transform(foo : diplomat_runtime::DiplomatStrSlice, write : &mut diplomat_runtime::DiplomatWrite) -> ();

    fn MyString_borrow(this: &MyString) -> diplomat_runtime::DiplomatStrSlice;

}