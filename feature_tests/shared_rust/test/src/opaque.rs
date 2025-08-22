#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{MyOpaqueEnum};

    #[test]
    fn test_opaque() {
        let e = MyOpaqueEnum::new();
        assert_eq!(String::from("MyOpaqueEnum::A"), e.to_string());
    }
}