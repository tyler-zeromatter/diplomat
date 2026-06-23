#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{Foo};
    #[test]
    fn test_lifetime_borrow() {
        let v = vec![0, 1, 2];
        let f = Foo::new(&v);
        // Uncomment to fail compilation, since `v` is borrowed for get_bar:
        // drop(v);
        let bar = f.get_bar();
        assert_eq!(*foo.as_returning().bytes, [0, 1, 2]);
    }
    // TODO: Test lifetime bounds.
}