#[repr(C)]
pub struct MyStructContainingAnOption {

}

impl MyStructContainingAnOption {
    pub fn new() -> MyStructContainingAnOption {
            // TODO: writeable conversions.
        unsafe { MyStructContainingAnOption_new() }
    }

    pub fn filled() -> MyStructContainingAnOption {
            // TODO: writeable conversions.
        unsafe { MyStructContainingAnOption_filled() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyStructContainingAnOption_new() -> MyStructContainingAnOption;

    fn MyStructContainingAnOption_filled() -> MyStructContainingAnOption;

}