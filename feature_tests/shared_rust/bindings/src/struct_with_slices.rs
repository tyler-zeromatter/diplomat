#[repr(C)]
pub struct StructWithSlices {
    pub first: &[TODO],
    pub second: &[u16],
}

impl StructWithSlices {
    pub fn return_last(self) -> String {
        let write = unsafe {
            diplomat_runtime::diplomat_buffer_write_create(0)
        };
        let ret = unsafe { StructWithSlices_return_last(self, write) };
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
    fn StructWithSlices_return_last(this : StructWithSlices, write : &mut diplomat_runtime::DiplomatWrite) -> String;

}