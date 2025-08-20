#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::MyEnum;
    #[test]
    fn test_enum() {
        assert!(matches!(MyEnum::get_a(), MyEnum::A));
        let e = MyEnum::B;
        assert_eq!(e.into_value(), -1);
    }
}