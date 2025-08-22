use super::CyclicStructB;
#[repr(C)]
pub struct CyclicStructA {
    pub a: CyclicStructB,
}

impl CyclicStructA {
    pub fn get_b() -> CyclicStructB {
        let ret = unsafe { CyclicStructA_get_b() };
        ret
    }

    pub fn cyclic_out(self) -> String {
        let write = diplomat_runtime::diplomat_buffer_write_create(0);
        let ret = unsafe { CyclicStructA_cyclic_out(self, write.as_mut().unwrap()) };
        // TODO: Create a helper in `lib.rs`.
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

    pub fn double_cyclic_out(self, cyclic_struct_a : CyclicStructA) -> String {
        let write = diplomat_runtime::diplomat_buffer_write_create(0);
        let ret = unsafe { CyclicStructA_double_cyclic_out(self, cyclic_struct_a, write.as_mut().unwrap()) };
        // TODO: Create a helper in `lib.rs`.
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

    pub fn getter_out(self) -> String {
        let write = diplomat_runtime::diplomat_buffer_write_create(0);
        let ret = unsafe { CyclicStructA_getter_out(self, write.as_mut().unwrap()) };
        // TODO: Create a helper in `lib.rs`.
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
    fn CyclicStructA_get_b() -> CyclicStructB;

    fn CyclicStructA_cyclic_out(this : CyclicStructA, write : &mut diplomat_runtime::DiplomatWrite) -> ();

    fn CyclicStructA_double_cyclic_out(this : CyclicStructA, cyclic_struct_a : CyclicStructA, write : &mut diplomat_runtime::DiplomatWrite) -> ();

    fn CyclicStructA_getter_out(this : CyclicStructA, write : &mut diplomat_runtime::DiplomatWrite) -> ();

}