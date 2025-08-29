pub struct MyString;

impl Drop for MyString {
    fn drop(&mut self) {
        unsafe { MyString_destroy(self) }
    }
}

impl MyString {
    pub fn new<'anon_0>(v : &'anon_0 [u8]) -> Box<MyString> {
        let ret = unsafe { MyString_new(v.into()) };
        
        ret
    
    }

    pub fn new_unsafe<'anon_0>(v : &'anon_0 str) -> Box<MyString> {
        let ret = unsafe { MyString_new_unsafe(v.into()) };
        
        ret
    
    }

    pub fn new_owned(v : Box<[u8]>) -> Box<MyString> {
        let ret = unsafe { MyString_new_owned(v.into()) };
        
        ret
    
    }

    pub fn new_from_first(v : &[&[u8]]) -> Box<MyString> {
        let ret = unsafe { MyString_new_from_first(v.iter().map(|u| { diplomat_runtime::DiplomatStrSlice::from(*u) }).collect::<Vec<_>>().as_slice().into()) };
        
        ret
    
    }

    pub fn set_str<'anon_0, 'anon_1>(&'anon_0 mut self, new_str : &'anon_1 [u8]) {
        unsafe { MyString_set_str(self, new_str.into()) };
        
    }

    pub fn get_str<'anon_0>(&'anon_0 self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        unsafe { MyString_get_str(self, write_mut) };
        
        let out_str = write.to_string();
        out_str
    
    }

    pub fn get_static_str() -> &'static str {
        let ret = unsafe { MyString_get_static_str() };
        
        ret.into()
    
    }

    pub fn string_transform<'anon_0>(foo : &'anon_0 str) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        unsafe { MyString_string_transform(foo.into(), write_mut) };
        
        let out_str = write.to_string();
        out_str
    
    }

    pub fn borrow<'a>(&'a self) -> &'a [u8] {
        let ret = unsafe { MyString_borrow(self) };
        
        ret.into()
    
    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn MyString_new<'anon_0>(v : diplomat_runtime::DiplomatStrSlice<'anon_0>) -> Box<MyString>;

    fn MyString_new_unsafe<'anon_0>(v : diplomat_runtime::DiplomatUtf8StrSlice<'anon_0>) -> Box<MyString>;

    fn MyString_new_owned(v : diplomat_runtime::DiplomatOwnedStrSlice) -> Box<MyString>;

    fn MyString_new_from_first(v : diplomat_runtime::DiplomatSlice<diplomat_runtime::DiplomatStrSlice>) -> Box<MyString>;

    fn MyString_set_str<'anon_0, 'anon_1>(this: &'anon_0 mut MyString, new_str : diplomat_runtime::DiplomatStrSlice<'anon_1>);

    fn MyString_get_str<'anon_0>(this: &'anon_0 MyString, write_mut : &mut crate::DiplomatWrite) -> ();

    fn MyString_get_static_str() -> diplomat_runtime::DiplomatUtf8StrSlice<'static>;

    fn MyString_string_transform<'anon_0>(foo : diplomat_runtime::DiplomatUtf8StrSlice<'anon_0>, write_mut : &mut crate::DiplomatWrite) -> ();

    fn MyString_borrow<'a>(this: &'a MyString) -> diplomat_runtime::DiplomatStrSlice<'a>;

    fn MyString_destroy(this : *mut MyString);
}