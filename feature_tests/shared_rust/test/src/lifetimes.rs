#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{Foo, BorrowedFieldsWithBounds};
    #[test]
    fn test_lifetime_borrow() {
        let v = vec![0, 1, 2];
        let f = Foo::new(&v);
        // Uncomment to fail compilation, since `v` is borrowed for get_bar:
        // drop(v);
        let bar = f.get_bar();
        assert_eq!(*bar.foo().as_returning().bytes, [0, 1, 2]);
    }

    #[test]
    // If you remove the bounds on the lifetimes below, this will fail to compile:
    fn test_bounds<'a, 'b : 'a, 'c: 'a + 'b>() {
        let v = vec![0, 1, 2];
        let foo = Foo::new(&v);
        let dstr16 = vec![3, 4, 5];
        let b = BorrowedFieldsWithBounds::<'a, 'b, 'c>::from_foo_and_strings(&foo, &dstr16, "utf8_str");
        let st : &'c str = b.field_c.into();
        assert_eq!(st, "utf8_str");
    }
}