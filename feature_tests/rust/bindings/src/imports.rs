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
        pub fn assert_eq(&self, c: u8) {
            unsafe { ImportedStruct_assert_eq(self, c) }
        }
    }
}
