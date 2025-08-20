pub enum UnimportedEnum {
    A, 
    B, 
    C
}

impl UnimportedEnum {
}

#[link(name = "somelib")]
unsafe extern "C" {
}