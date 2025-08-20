use super::MyEnum;
use super::MyZst;
#[repr(C)]
pub struct MyStruct {
    pub a: u8,
    pub b: bool,
    pub c: u8,
    pub d: u64,
    pub e: i32,
    pub f: char,
    pub g: MyEnum,
}

impl MyStruct {
    pub fn new() -> MyStruct {
            // TODO: writeable conversions.
        unsafe { MyStruct_new() }
    }

    pub fn takes_mut(&mut self, o : MyStruct) {
            // TODO: writeable conversions.
        unsafe { MyStruct_takes_mut(self, o) }
    }

    pub fn takes_const(&self, o : MyStruct) {
            // TODO: writeable conversions.
        unsafe { MyStruct_takes_const(self, o) }
    }

    pub fn into_a(self) -> u8 {
            // TODO: writeable conversions.
        unsafe { MyStruct_into_a(self) }
    }

    pub fn returns_zst_result() -> Result<(), MyZst> {
            // TODO: writeable conversions.
        unsafe { MyStruct_returns_zst_result() }
    }

    pub fn fails_zst_result() -> Result<(), MyZst> {
            // TODO: writeable conversions.
        unsafe { MyStruct_fails_zst_result() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyStruct_new() -> MyStruct;

    fn MyStruct_takes_mut(this: &mut MyStruct, o : MyStruct);

    fn MyStruct_takes_const(this: &MyStruct, o : MyStruct);

    fn MyStruct_into_a(this : MyStruct) -> u8;

    fn MyStruct_returns_zst_result() -> Result<(), MyZst>;

    fn MyStruct_fails_zst_result() -> Result<(), MyZst>;

}