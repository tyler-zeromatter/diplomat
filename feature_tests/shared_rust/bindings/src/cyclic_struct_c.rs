use super::CyclicStructA;
#[repr(C)]
pub struct CyclicStructC {
    pub a: CyclicStructA,
}

impl CyclicStructC {
    pub fn takes_nested_parameters(c : CyclicStructC) -> CyclicStructC {
        let ret = unsafe { CyclicStructC_takes_nested_parameters(c) };
        ret
    }

    pub fn cyclic_out(self) -> String {
        let write = diplomat_runtime::diplomat_buffer_write_create(0);
        let ret = unsafe { CyclicStructC_cyclic_out(self, write.as_mut().unwrap()) };
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
        
        // Drop the write object, since we no longer need it:
        unsafe {
            drop(Box::from_raw(write))
        }
        out_str
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn CyclicStructC_takes_nested_parameters(c : CyclicStructC) -> CyclicStructC;

    fn CyclicStructC_cyclic_out(this : CyclicStructC, write : &mut diplomat_runtime::DiplomatWrite) -> ();
}