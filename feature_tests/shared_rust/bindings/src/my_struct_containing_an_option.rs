use super::DefaultEnum;
use super::MyStruct;
#[repr(C)]
pub struct MyStructContainingAnOption {
    pub a: Option<MyStruct>,
    pub b: Option<DefaultEnum>,
}

impl MyStructContainingAnOption {
    pub fn new() -> MyStructContainingAnOption {
        let ret = unsafe { MyStructContainingAnOption_new() };
        
        ret
    
    }

    pub fn filled() -> MyStructContainingAnOption {
        let ret = unsafe { MyStructContainingAnOption_filled() };
        
        ret
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn MyStructContainingAnOption_new() -> MyStructContainingAnOption;

    fn MyStructContainingAnOption_filled() -> MyStructContainingAnOption;
}