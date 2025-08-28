#[repr(C)]
pub struct PrimitiveStruct {
    pub x: f32,
    pub a: bool,
    pub b: diplomat_runtime::DiplomatChar,
    pub c: i64,
    pub d: isize,
    pub e: u8,
}

impl PrimitiveStruct {
    pub fn mutable_ref<'anon_0, 'anon_1>(&'anon_0 mut self, a : &'anon_1 mut PrimitiveStruct) {
        let ret = unsafe { PrimitiveStruct_mutable_ref(self, a) };
        
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn PrimitiveStruct_mutable_ref<'anon_0, 'anon_1>(this: &'anon_0 mut PrimitiveStruct, a : &'anon_1 mut PrimitiveStruct);
}