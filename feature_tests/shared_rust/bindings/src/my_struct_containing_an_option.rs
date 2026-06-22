use super::DefaultEnum;
use super::MyStruct;
use super::my_struct::MyStructAbi;
#[repr(C)]
pub struct MyStructContainingAnOption {
    pub a: diplomat_runtime::DiplomatOption::<MyStructAbi>,
    pub b: diplomat_runtime::DiplomatOption::<DefaultEnum>,
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