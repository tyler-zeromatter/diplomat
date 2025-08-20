#[repr(C)]
pub struct PrimitiveStruct {

}

impl PrimitiveStruct {
    fn mutable_ref(&mut self, a : PrimitiveStruct) {
            // TODO: writeable conversions.
        unsafe { PrimitiveStruct_mutable_ref(self, a) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn PrimitiveStruct_mutable_ref(this: &mut PrimitiveStruct, a : PrimitiveStruct);

}