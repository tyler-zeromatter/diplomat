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
        let write = diplomat_runtime::diplomat_buffer_write_create(0);
        let ret = unsafe { MyOpaqueEnum_to_string(self, write.as_mut().unwrap()) };
        // TODO: Create a helper in `lib.rs`.
        let out_str = unsafe {
            let write_ref = write.as_ref().unwrap();
            let buf = diplomat_runtime::diplomat_buffer_write_get_bytes(write_ref);
            let len = diplomat_runtime::diplomat_buffer_write_len(write_ref);
    
            if !buf.is_null() {
                // String takes ownership of the buffer:
                String::from_raw_parts(buf, len, len)
            } else {
                panic!("Could not read buffer, growth failed.")
            }
        };
        out_str
    }

    

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyOpaqueEnum_new() -> Box<MyOpaqueEnum>;

    fn MyOpaqueEnum_to_string(this: &MyOpaqueEnum, write : &mut diplomat_runtime::DiplomatWrite) -> ();

    fn MyOpaqueEnum_destroy(this : *mut MyOpaqueEnum);

}