#[repr(C)]
pub struct MyStructContainingAnOption {

}

impl MyStructContainingAnOption {
    fn new() -> MyStructContainingAnOption {
            // TODO: writeable conversions.
        unsafe { MyStructContainingAnOption_new() }
    }

    fn filled() -> MyStructContainingAnOption {
            // TODO: writeable conversions.
        unsafe { MyStructContainingAnOption_filled() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyStructContainingAnOption_new() -> MyStructContainingAnOption;

    fn MyStructContainingAnOption_filled() -> MyStructContainingAnOption;

}