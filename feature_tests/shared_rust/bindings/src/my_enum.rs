#[repr(C)]
pub enum MyEnum {
    A = -2, 
    B = -1, 
    C = 0, 
    D = 1, 
    E = 2, 
    F = 3
}

impl MyEnum {
    pub fn into_value(self) -> i8 {
        let ret = unsafe { MyEnum_into_value(self) };
        
        ret
    
    }

    pub fn get_a() -> MyEnum {
        let ret = unsafe { MyEnum_get_a() };
        
        ret
    
    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn MyEnum_into_value(this : MyEnum) -> i8;

    fn MyEnum_get_a() -> MyEnum;
}