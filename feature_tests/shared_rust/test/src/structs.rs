#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{MyEnum, MyStruct};

    #[test]
    fn test_struct_invariants() {
        let st = MyStruct::new();
        assert_eq!(st.a, 17);
        assert_eq!(st.b, true);
        assert_eq!(st.c, 209);
        assert_eq!(st.d, 1234);
        assert_eq!(st.e, 5991);
        assert_eq!(st.f, '餐');
        assert!(matches!(st.g, MyEnum::B));
        assert_eq!(st.into_a(), 17);
    }
}