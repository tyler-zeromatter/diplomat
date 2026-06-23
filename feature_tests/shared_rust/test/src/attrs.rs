#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{AttrOpaque1Renamed};
    #[test]
    fn test_rename() {
        let op = AttrOpaque1Renamed::new();
        assert_eq!(op.method(), 77);
    }
}