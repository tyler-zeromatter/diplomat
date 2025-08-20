use super::MyZst;
#[repr(C)]
pub struct MyStruct {

}

impl MyStruct {
    fn new() -> MyStruct {
            // TODO: writeable conversions.
        unsafe { MyStruct_new() }
    }

    fn takes_mut(&mut self, o : MyStruct) {
            // TODO: writeable conversions.
        unsafe { MyStruct_takes_mut(self, o) }
    }

    fn takes_const(&self, o : MyStruct) {
            // TODO: writeable conversions.
        unsafe { MyStruct_takes_const(self, o) }
    }

    fn into_a(mut self) -> u8 {
            // TODO: writeable conversions.
        unsafe { MyStruct_into_a(self) }
    }

    fn returns_zst_result() -> Result<(), MyZst> {
            // TODO: writeable conversions.
        unsafe { MyStruct_returns_zst_result() }
    }

    fn fails_zst_result() -> Result<(), MyZst> {
            // TODO: writeable conversions.
        unsafe { MyStruct_fails_zst_result() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyStruct_new() -> MyStruct;

    fn MyStruct_takes_mut(this: &mut MyStruct, o : MyStruct);

    fn MyStruct_takes_const(this: &MyStruct, o : MyStruct);

    fn MyStruct_into_a(mut this : MyStruct) -> u8;

    fn MyStruct_returns_zst_result() -> Result<(), MyZst>;

    fn MyStruct_fails_zst_result() -> Result<(), MyZst>;

}