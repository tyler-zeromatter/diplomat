pub struct PrimitiveStruct {
    pub x: f32,
    pub a: bool,
    pub b: diplomat_runtime::DiplomatChar,
    pub c: i64,
    pub d: isize,
    pub e: u8,
}

#[repr(C)]
pub(crate) struct PrimitiveStructAbi {
    
    x : f32,
    
    a : bool,
    
    b : diplomat_runtime::DiplomatChar,
    
    c : i64,
    
    d : isize,
    
    e : u8,
    
}

impl PrimitiveStructAbi {
    fn from_ffi(self) -> PrimitiveStruct{
        PrimitiveStruct {
            
                x: self.x,
            
                a: self.a,
            
                b: self.b,
            
                c: self.c,
            
                d: self.d,
            
                e: self.e,
            
        }
    }
}

impl PrimitiveStruct {
    pub fn mutable_ref<'anon_0, 'anon_1>(&'anon_0 mut self, a : &'anon_1 mut PrimitiveStruct) {
        let ret = unsafe { PrimitiveStruct_mutable_ref(self, a) };
        
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn PrimitiveStruct_mutable_ref<'anon_0, 'anon_1>(this: &'anon_0 mut PrimitiveStruct, a : &'anon_1 mut PrimitiveStructAbi);
}