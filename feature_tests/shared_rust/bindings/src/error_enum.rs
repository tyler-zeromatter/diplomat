#[repr(C)]
pub enum ErrorEnum {
    Foo = 0, 
    Bar = 1
}

impl ErrorEnum {
}

#[link(name = "somelib")]
unsafe extern "C" {}