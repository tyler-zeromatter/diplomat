#[repr(C)]
pub struct MyStructContainingAnOption {

}

impl MyStructContainingAnOption {
    fn new() -> MyStructContainingAnOption {
        unsafe { MyStructContainingAnOption_new() }
    }

    fn filled() -> MyStructContainingAnOption {
        unsafe { MyStructContainingAnOption_filled() }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn MyStructContainingAnOption_new() -> MyStructContainingAnOption;

    fn MyStructContainingAnOption_filled() -> MyStructContainingAnOption;

}