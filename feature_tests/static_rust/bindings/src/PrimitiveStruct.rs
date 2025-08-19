#[repr(C)]
pub struct PrimitiveStruct {

}

impl PrimitiveStruct {
    fn mutable_ref(&self, a : PrimitiveStruct) {
        unsafe { PrimitiveStruct_mutable_ref(self, a) }
    }

}

#[link(name = "somelib")]
extern "C" {
    fn PrimitiveStruct_mutable_ref(&self, a : PrimitiveStruct);

}