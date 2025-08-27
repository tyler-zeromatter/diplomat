pub struct MyString;

impl Drop for MyString {
    fn drop(&mut self) {
        unsafe { MyString_destroy(self) }
    }
}

impl MyString {
    pub fn new(v : &'anon_0 [u8]) -> Box<MyString> {
        let ret = unsafe { MyString_new(v.into()) };
        ret
    }

    pub fn new_unsafe(v : &'anon_0 String) -> Box<MyString> {
        let ret = unsafe { MyString_new_unsafe(v.into()) };
        ret
    }

    pub fn new_owned(v : Box<[u8]>) -> Box<MyString> {
        let ret = unsafe { MyString_new_owned(v.into()) };
        ret
    }

    pub fn new_from_first(v : &[String]) -> Box<MyString> {
        let ret = unsafe { MyString_new_from_first(v) };
        ret
    }

    pub fn set_str(&mut self, new_str : &'anon_1 [u8]) {
        let ret = unsafe { MyString_set_str(self, new_str.into()) };
        ret
    }

    pub fn get_str(&self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { MyString_get_str(self, write_mut) };
        let out_str = write.to_string();
        out_str
    }

    pub fn get_static_str() -> &'static String {
        let ret = unsafe { MyString_get_static_str() };
        ret
    }

    pub fn string_transform(foo : &'anon_0 String) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { MyString_string_transform(foo.into(), write_mut) };
        let out_str = write.to_string();
        out_str
    }

    pub fn borrow(&self) -> &'a [u8] {
        let ret = unsafe { MyString_borrow(self) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn MyString_new(v : &'anon_0 diplomat_runtime::DiplomatStrSlice) -> Box<MyString>;

    fn MyString_new_unsafe(v : &'anon_0 diplomat_runtime::DiplomatStrSlice) -> Box<MyString>;

    fn MyString_new_owned(v : diplomat_runtime::DiplomatStrSlice) -> Box<MyString>;

    fn MyString_new_from_first(v : &[String]) -> Box<MyString>;

    fn MyString_set_str(this: &mut MyString, new_str : &'anon_1 diplomat_runtime::DiplomatStrSlice);

    fn MyString_get_str(this: &MyString, write_mut : &mut crate::DiplomatWrite) -> ();

    fn MyString_get_static_str() -> &'static diplomat_runtime::DiplomatStrSlice;

    fn MyString_string_transform(foo : &'anon_0 diplomat_runtime::DiplomatStrSlice, write_mut : &mut crate::DiplomatWrite) -> ();

    fn MyString_borrow(this: &MyString) -> &'a diplomat_runtime::DiplomatStrSlice;

    fn MyString_destroy(this : *mut MyString);
}