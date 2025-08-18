#[diplomat::bridge]
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
            Self {
                foo: UnimportedEnum::A,
                count : 5
            }
        }

        pub fn assert_eq(&self, c : u8) {
            assert_eq!(self.count, c);
        }
    }
}
