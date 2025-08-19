#[repr(C)]
pub struct MyStruct {

}

impl MyStruct {
    fn new() {
        unsafe { MyStruct_new() }
    }

    fn into_a() {
        unsafe { MyStruct_into_a() }
    }

    fn returns_zst_result() {
        unsafe { MyStruct_returns_zst_result() }
    }

    fn fails_zst_result() {
        unsafe { MyStruct_fails_zst_result() }
    }

}