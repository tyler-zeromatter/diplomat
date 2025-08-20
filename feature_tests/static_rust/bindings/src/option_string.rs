pub struct OptionString;

impl OptionString {
    fn new(diplomat_str : TODO()) -> OptionString {
            // TODO: writeable conversions.
        unsafe { OptionString_new(diplomat_str) }
    }

    fn write(&self) -> Result<(), ()> {
            // TODO: writeable conversions.
        unsafe { OptionString_write(self, output) }
    }

    fn borrow(&self) -> Option<TODO()> {
            // TODO: writeable conversions.
        unsafe { OptionString_borrow(self) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OptionString_new(diplomat_str : TODO()) -> OptionString;

    fn OptionString_write(this: &OptionString, output : &mut DiplomatWrite) -> Result<(), ()>;

    fn OptionString_borrow(this: &OptionString) -> Option<TODO()>;

}