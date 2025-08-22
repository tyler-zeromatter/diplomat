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

    pub fn takes_mut(&mut self, o : MyStruct) {
        let ret = unsafe { MyStruct_takes_mut(self, o) };
        ret
    }

    pub fn takes_const(&self, o : MyStruct) {
        let ret = unsafe { MyStruct_takes_const(self, o) };
        ret
    }

    pub fn into_a(self) -> u8 {
        let ret = unsafe { MyStruct_into_a(self) };
        ret
    }

    pub fn returns_zst_result() -> Result<(), MyZst> {
        let ret = unsafe { MyStruct_returns_zst_result() };
        ret.into()
    }

    pub fn fails_zst_result() -> Result<(), MyZst> {
        let ret = unsafe { MyStruct_fails_zst_result() };
        ret.into()
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyStruct_new() -> MyStruct;

    fn MyStruct_takes_mut(this: &mut MyStruct, o : MyStruct);

    fn MyStruct_takes_const(this: &MyStruct, o : MyStruct);

    fn MyStruct_into_a(this : MyStruct) -> u8;

    fn MyStruct_returns_zst_result() -> diplomat_runtime::DiplomatResult<(), MyZst>;

    fn MyStruct_fails_zst_result() -> diplomat_runtime::DiplomatResult<(), MyZst>;

}