use super::MyEnum;
use super::MyZst;
#[repr(C)]
pub struct MyStruct {
    pub a: u8,
    pub b: bool,
    pub c: u8,
    pub d: u64,
    pub e: i32,
    pub f: diplomat_runtime::DiplomatChar,
    pub g: MyEnum,
}

impl MyStruct {
    pub fn new() -> MyStruct {
        let ret = unsafe { MyStruct_new() };
        
        ret
    
    }

    pub fn takes_mut<'anon_0, 'anon_1>(&'anon_0 mut self, o : &'anon_1 mut MyStruct) {
        let ret = unsafe { MyStruct_takes_mut(self, o) };
        
    }

    pub fn takes_const<'anon_0, 'anon_1>(&'anon_0 self, o : &'anon_1 mut MyStruct) {
        let ret = unsafe { MyStruct_takes_const(self, o) };
        
    }

    pub fn into_a(self) -> u8 {
        let ret = unsafe { MyStruct_into_a(self) };
        
        ret
    
    }

    pub fn returns_zst_result() -> Result<(), MyZst> {
        let ret = unsafe { MyStruct_returns_zst_result() };
        
        ret.to_result()
    
    }

    pub fn fails_zst_result() -> Result<(), MyZst> {
        let ret = unsafe { MyStruct_fails_zst_result() };
        
        ret.to_result()
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn MyStruct_new() -> MyStruct;

    fn MyStruct_takes_mut<'anon_0, 'anon_1>(this: &'anon_0 mut MyStruct, o : &'anon_1 mut MyStruct);

    fn MyStruct_takes_const<'anon_0, 'anon_1>(this: &'anon_0 MyStruct, o : &'anon_1 mut MyStruct);

    fn MyStruct_into_a(this : MyStruct) -> u8;

    fn MyStruct_returns_zst_result() -> crate::DiplomatResult<(), MyZst>;

    fn MyStruct_fails_zst_result() -> crate::DiplomatResult<(), MyZst>;
}