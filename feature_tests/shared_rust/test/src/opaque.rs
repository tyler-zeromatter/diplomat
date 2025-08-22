#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{MyOpaqueEnum};

    #[test]
    fn test_opaque() {
        let e = MyOpaqueEnum::new();
        // TODO: Valgrind has issues with this as well:
        let a = e.to_string();
        assert_eq!(String::from("MyOpaqueEnum::A"), a);
    }
}