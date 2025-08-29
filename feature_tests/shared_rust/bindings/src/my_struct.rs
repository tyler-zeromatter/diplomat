use super::MyEnum;
use super::MyZst;
use super::my_zst::MyZstAbi;
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
    pub(crate) fn from_ffi(self) -> MyStruct{
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

    pub (crate) fn to_ffi(this : MyStruct) -> MyStructAbi{
        MyStructAbi {
            
            a : this.a,
            
            b : this.b,
            
            c : this.c,
            
            d : this.d,
            
            e : this.e,
            
            f : this.f,
            
            g : this.g,
            
        }
    }
}

impl From<MyStruct> for MyStructAbi{
    fn from(value: MyStruct) -> Self {
        MyStructAbi::to_ffi(value)
    }
}

impl From<MyStructAbi> for MyStruct{
    fn from(value: MyStructAbi) -> Self {
        MyStructAbi::from_ffi(value)
    }
}

impl MyStruct {
    pub fn new() -> MyStruct {
        let ret = unsafe { MyStruct_new() };
        
        ret.from_ffi()
    
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

    fn MyStruct_into_a(this : MyStruct) -> u8;

    fn MyStruct_returns_zst_result() -> crate::DiplomatResult<(), MyZstAbi>;

    fn MyStruct_fails_zst_result() -> crate::DiplomatResult<(), MyZstAbi>;
}