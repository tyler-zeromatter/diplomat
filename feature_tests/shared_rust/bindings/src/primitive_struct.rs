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
    pub(crate) fn from_ffi(self) -> PrimitiveStruct{
        PrimitiveStruct {
            
            x: self.x,
            
            a: self.a,
            
            b: self.b,
            
            c: self.c,
            
            d: self.d,
            
            e: self.e,
            
        }
    }

    pub (crate) fn to_ffi(this : PrimitiveStruct) -> PrimitiveStructAbi{
        PrimitiveStructAbi {
            
            x : this.x,
            
            a : this.a,
            
            b : this.b,
            
            c : this.c,
            
            d : this.d,
            
            e : this.e,
            
        }
    }
}

impl From<PrimitiveStruct> for PrimitiveStructAbi{
    fn from(value: PrimitiveStruct) -> Self {
        PrimitiveStructAbi::to_ffi(value)
    }
}

impl From<PrimitiveStructAbi> for PrimitiveStruct{
    fn from(value: PrimitiveStructAbi) -> Self {
        PrimitiveStructAbi::from_ffi(value)
    }
}

impl PrimitiveStruct {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}