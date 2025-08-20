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
            // TODO: writeable conversions.
        unsafe { MyEnum_into_value(self) }
    }

    pub fn get_a() -> MyEnum {
            // TODO: writeable conversions.
        unsafe { MyEnum_get_a() }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn MyEnum_into_value(this : MyEnum) -> i8;

    fn MyEnum_get_a() -> MyEnum;

}