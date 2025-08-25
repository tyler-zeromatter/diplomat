pub struct MyOpaqueEnum;

impl Drop for MyOpaqueEnum {
    fn drop(&mut self) {
        unsafe { MyOpaqueEnum_destroy(self) }
    }
}

impl MyOpaqueEnum {
    pub fn new() -> Box<MyOpaqueEnum> {
        let ret = unsafe { MyOpaqueEnum_new() };
        ret
    }

    pub fn to_string(&self) -> String {
        let mut write = crate::DiplomatWrite::new();
        let write_mut = &mut write;
        let ret = unsafe { MyOpaqueEnum_to_string(self, write_mut) };
        let out_str = write.to_string();
        out_str
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn MyOpaqueEnum_new() -> Box<MyOpaqueEnum>;

    fn MyOpaqueEnum_to_string(this: &MyOpaqueEnum, write_mut : &mut crate::DiplomatWrite) -> ();

    fn MyOpaqueEnum_destroy(this : *mut MyOpaqueEnum);
}