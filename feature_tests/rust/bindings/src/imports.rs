#[diplomat_static_rust::bridge]
pub mod ffi {
    pub enum UnimportedEnum {
        A,
        B,
        C,
    }
    pub struct ImportedStruct {
        foo: UnimportedEnum,
        count: u8,
    }
    impl ImportedStruct {
        pub fn small_test() -> Self {
            unsafe { ImportedStruct_small_test() }
        }
    }
}
