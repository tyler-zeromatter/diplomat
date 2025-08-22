#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{MyEnum, OptionString};
    #[test]
    fn test_enum() {
        assert!(matches!(MyEnum::get_a(), MyEnum::A));
        let e = MyEnum::B;
        assert_eq!(e.into_value(), -1);
    }

    #[test]
    fn test_string() {
        let string = format!("This is a test");
        let opt = OptionString::new(string.as_bytes()).unwrap();
        assert_eq!(opt.write().unwrap(), string)
    }
}