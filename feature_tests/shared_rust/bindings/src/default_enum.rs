#[repr(C)]
pub enum DefaultEnum {
    A = 0, 
    B = 1
}

impl DefaultEnum {
    pub fn new() -> DefaultEnum {
        let ret = unsafe { DefaultEnum_new() };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn DefaultEnum_new() -> DefaultEnum;
}