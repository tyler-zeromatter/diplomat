#[repr(C)]
pub struct MyStructContainingAnOption {

}

impl MyStructContainingAnOption {
    fn new() {
        unsafe { MyStructContainingAnOption_new() }
    }

    fn filled() {
        unsafe { MyStructContainingAnOption_filled() }
    }

}