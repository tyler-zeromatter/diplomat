#[repr(C)]
pub struct PrimitiveStruct {
    pub x: f32,
    pub a: bool,
    pub b: diplomat_runtime::DiplomatChar,
    pub c: i64,
    pub d: isize,
    pub e: byte,
}

impl PrimitiveStruct {
    pub fn mutable_ref(&mut self, a : PrimitiveStruct) {
        let ret = unsafe { PrimitiveStruct_mutable_ref(self, a) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn PrimitiveStruct_mutable_ref(this: &mut PrimitiveStruct, a : PrimitiveStruct);
}