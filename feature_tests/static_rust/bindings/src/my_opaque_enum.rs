pub struct MyOpaqueEnum;

impl MyOpaqueEnum {
    pub fn new() -> Box<MyOpaqueEnum> {
            // TODO: writeable conversions.
        unsafe { MyOpaqueEnum_new() }
    }

    pub fn to_string(&self) {
            // TODO: writeable conversions.
        unsafe { MyOpaqueEnum_to_string(self, output) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyOpaqueEnum_new() -> Box<MyOpaqueEnum>;

    fn MyOpaqueEnum_to_string(this: &MyOpaqueEnum, output : &mut DiplomatWrite);

}