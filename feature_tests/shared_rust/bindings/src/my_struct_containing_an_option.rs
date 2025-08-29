use super::DefaultEnum;
use super::MyStruct;
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
    fn from_ffi(self) -> MyStructContainingAnOption{
        MyStructContainingAnOption {
            
                a: self.a,
            
                b: self.b,
            
        }
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