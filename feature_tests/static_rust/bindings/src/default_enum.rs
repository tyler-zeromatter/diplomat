pub enum DefaultEnum {
    A, 
    B
}

impl DefaultEnum {
    fn new() -> DefaultEnum {
            // TODO: writeable conversions.
        unsafe { DefaultEnum_new() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn DefaultEnum_new() -> DefaultEnum;

}