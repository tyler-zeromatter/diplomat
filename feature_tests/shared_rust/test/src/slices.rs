#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{OptionString};

    #[test]
    fn test_string() {
        let string = format!("This is a test");
        let opt = OptionString::new(string.as_bytes()).unwrap();
        assert_eq!(opt.write().unwrap(), string)
    }
}