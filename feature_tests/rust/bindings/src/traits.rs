#[diplomat_static_rust::bridge(lib_name = "somelib")]
mod ffi {
    pub struct TraitTestingStruct {
        x: i32,
        y: i32,
    }
    pub trait TesterTrait {
        fn test_trait_fn(&self, x: u32) -> u32;
        fn test_void_trait_fn(&self);
        fn test_struct_trait_fn(&self, s: TraitTestingStruct) -> i32;
        #[diplomat::attr(kotlin, disable)]
        fn test_result_output(&self) -> Result<u32, ()>;
    }
    pub struct TraitWrapper {
        cant_be_empty: bool,
    }
    impl TraitWrapper {
        pub fn test_with_trait(t: impl TesterTrait, x: i32) -> i32 {
            unsafe { TraitWrapper_test_with_trait(t, x) }
        }
        pub fn test_trait_with_struct(t: impl TesterTrait) -> i32 {
            unsafe { TraitWrapper_test_trait_with_struct(t) }
        }
        pub fn test_result_output(t: impl TesterTrait) {
            unsafe { TraitWrapper_test_result_output(t) }
        }
    }
}
