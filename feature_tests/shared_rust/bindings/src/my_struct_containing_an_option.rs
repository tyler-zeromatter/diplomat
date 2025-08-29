use super::DefaultEnum;
use super::MyStruct;
use super::my_struct::MyStructAbi;
pub struct MyStructContainingAnOption {
    pub a: Option<MyStruct>,
    pub b: Option<DefaultEnum>,
}

#[repr(C)]
pub(crate) struct MyStructContainingAnOptionAbi {
    a : diplomat_runtime::DiplomatOption<MyStructAbi>,
    b : diplomat_runtime::DiplomatOption<DefaultEnum>,
}

impl MyStructContainingAnOptionAbi {
    pub(crate) fn from_ffi(self) -> MyStructContainingAnOption{
        MyStructContainingAnOption {
            
            a: self.a.into_converted_option(),
            
            b: self.b.into_converted_option(),
            
        }
    }

    pub (crate) fn to_ffi(this : MyStructContainingAnOption) -> MyStructContainingAnOptionAbi{
        MyStructContainingAnOptionAbi {
            
            a : this.a.map(|ok| { ok.into() }).into(),
            
            b : this.b.into(),
            
        }
    }
}

impl From<MyStructContainingAnOption> for MyStructContainingAnOptionAbi{
    fn from(value: MyStructContainingAnOption) -> Self {
        MyStructContainingAnOptionAbi::to_ffi(value)
    }
}

impl From<MyStructContainingAnOptionAbi> for MyStructContainingAnOption{
    fn from(value: MyStructContainingAnOptionAbi) -> Self {
        value.from_ffi()
    }
}

impl MyStructContainingAnOption {
    pub fn new() -> MyStructContainingAnOption {
        let ret = unsafe { MyStructContainingAnOption_new() };
        
        ret.from_ffi()
    
    }

    pub fn filled() -> MyStructContainingAnOption {
        let ret = unsafe { MyStructContainingAnOption_filled() };
        
        ret.from_ffi()
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn MyStructContainingAnOption_new() -> MyStructContainingAnOptionAbi;

    fn MyStructContainingAnOption_filled() -> MyStructContainingAnOptionAbi;
}