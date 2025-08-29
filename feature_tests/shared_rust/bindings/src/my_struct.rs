use super::MyEnum;
use super::MyZst;
pub struct MyStruct {
    pub a: u8,
    pub b: bool,
    pub c: u8,
    pub d: u64,
    pub e: i32,
    pub f: diplomat_runtime::DiplomatChar,
    pub g: MyEnum,
}

#[repr(C)]
pub(crate) struct MyStructAbi {
    
    a : u8,
    
    b : bool,
    
    c : u8,
    
    d : u64,
    
    e : i32,
    
    f : diplomat_runtime::DiplomatChar,
    
    g : MyEnum,
    
}

impl MyStructAbi {
    fn from_ffi(self) -> MyStruct{
        MyStruct {
            
                a: self.a,
            
                b: self.b,
            
                c: self.c,
            
                d: self.d,
            
                e: self.e,
            
                f: self.f,
            
                g: self.g,
            
        }
    }
}

impl MyStruct {
    pub fn new() -> MyStruct {
        let ret = unsafe { MyStruct_new() };
        
        ret.from_ffi()
    
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
        
        ret.to_result().map_err(|err : MyZstAbi| { err.from_ffi() })
    
    }

    pub fn fails_zst_result() -> Result<(), MyZst> {
        let ret = unsafe { MyStruct_fails_zst_result() };
        
        ret.to_result().map_err(|err : MyZstAbi| { err.from_ffi() })
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn MyStruct_new() -> MyStructAbi;

    fn MyStruct_takes_mut<'anon_0, 'anon_1>(this: &'anon_0 mut MyStruct, o : &'anon_1 mut MyStructAbi);

    fn MyStruct_takes_const<'anon_0, 'anon_1>(this: &'anon_0 MyStruct, o : &'anon_1 mut MyStructAbi);

    fn MyStruct_into_a(this : MyStruct) -> u8;

    fn MyStruct_returns_zst_result() -> crate::DiplomatResult<(), MyZstAbi>;

    fn MyStruct_fails_zst_result() -> crate::DiplomatResult<(), MyZstAbi>;
}