pub struct OpaqueThin;

impl OpaqueThin {
    pub fn a(&self) -> i32 {
        let ret = unsafe { OpaqueThin_a(self) };
        ret
    }

    pub fn b(&self) -> f32 {
        let ret = unsafe { OpaqueThin_b(self) };
        ret
    }

    pub fn c(&self) -> String {
        let write = unsafe {
            diplomat_runtime::diplomat_buffer_write_create(0)
        };
        let ret = unsafe { OpaqueThin_c(self, write.as_mut().unwrap()) };
        let out_str = unsafe {
            let write_ref = write.as_ref().unwrap();
            let buf = diplomat_runtime::diplomat_buffer_write_get_bytes(write_ref);
            let len = diplomat_runtime::diplomat_buffer_write_len(write_ref);
    
            if !buf.is_null() {
                String::from_raw_parts(buf, len, len)
            } else {
                panic!("Could not read buffer, growth failed.")
            }
        };
    
        unsafe {
            diplomat_runtime::diplomat_buffer_write_destroy(write);
        }
        out_str
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OpaqueThin_a(this: &OpaqueThin) -> i32;

    fn OpaqueThin_b(this: &OpaqueThin) -> f32;

    fn OpaqueThin_c(this: &OpaqueThin, write : &mut diplomat_runtime::DiplomatWrite) -> String;

}