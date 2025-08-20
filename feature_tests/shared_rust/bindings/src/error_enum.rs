#[repr(C)]
pub enum ErrorEnum {
    Foo, 
    Bar
}

impl ErrorEnum {
}

#[link(name = "somelib")]
unsafe extern "C" {
}