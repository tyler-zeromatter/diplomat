#[repr(C)]
pub enum DefaultEnum {
    A = 0, 
    B = 1
}

impl DefaultEnum {
    pub fn new() -> DefaultEnum {
            // TODO: writeable conversions.
        unsafe { DefaultEnum_new() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn DefaultEnum_new() -> DefaultEnum;

}