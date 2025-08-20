pub struct OptionString;

impl OptionString {
    pub fn new(diplomat_str : &[TODO]) -> Box<Option<OptionString>> {
            // TODO: writeable conversions.
        unsafe { OptionString_new(diplomat_str) }
    }

    pub fn write(&self) -> Result<(), ()> {
            // TODO: writeable conversions.
        unsafe { OptionString_write(self, output) }
    }

    pub fn borrow(&self) -> Option<&[TODO]> {
            // TODO: writeable conversions.
        unsafe { OptionString_borrow(self) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OptionString_new(diplomat_str : &[TODO]) -> Box<Option<OptionString>>;

    fn OptionString_write(this: &OptionString, output : &mut DiplomatWrite) -> Result<(), ()>;

    fn OptionString_borrow(this: &OptionString) -> Option<&[TODO]>;

}