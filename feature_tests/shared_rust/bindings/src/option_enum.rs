#[repr(C)]
pub enum OptionEnum {
    Foo = 0, 
    Bar = 1
}

impl OptionEnum {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {}