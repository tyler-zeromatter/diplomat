use super::MyZst;
#[repr(C)]
pub struct MyStruct {

}

impl MyStruct {
    fn new() -> MyStruct {
        unsafe { MyStruct_new() }
    }

    fn takes_mut(&self, o : MyStruct) {
        unsafe { MyStruct_takes_mut(self, o) }
    }

    fn takes_const(&mut self, o : MyStruct) {
        unsafe { MyStruct_takes_const(self, o) }
    }

    fn into_a(self) -> u8 {
        unsafe { MyStruct_into_a(self) }
    }

    fn returns_zst_result() -> Result<(), MyZst> {
        unsafe { MyStruct_returns_zst_result() }
    }

    fn fails_zst_result() -> Result<(), MyZst> {
        unsafe { MyStruct_fails_zst_result() }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn MyStruct_new() -> MyStruct;

    fn MyStruct_takes_mut(&self, o : MyStruct);

    fn MyStruct_takes_const(&mut self, o : MyStruct);

    fn MyStruct_into_a(self) -> u8;

    fn MyStruct_returns_zst_result() -> Result<(), MyZst>;

    fn MyStruct_fails_zst_result() -> Result<(), MyZst>;

}