#[repr(C)]
pub enum UnimportedEnum {
    A = 0, 
    B = 1, 
    C = 2
}

impl UnimportedEnum {
}

#[link(name = "somelib")]
unsafe extern "C" {
}