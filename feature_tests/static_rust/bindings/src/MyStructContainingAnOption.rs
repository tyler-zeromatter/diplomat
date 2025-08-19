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

#[link(name = "somelib")]
extern "C" {
    fn MyStructContainingAnOption_new();

    fn MyStructContainingAnOption_filled();

}