use super::UnimportedEnum;
#[repr(C)]
pub struct ImportedStruct {
    pub foo: UnimportedEnum,
    pub count: u8,
}

impl ImportedStruct {
}

#[link(name = "somelib")]
unsafe extern "C" {
}