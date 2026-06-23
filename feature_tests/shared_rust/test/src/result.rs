#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{ErrorEnum, MyStruct, MyZst, ResultOpaque, ErrorStruct};

    #[test]
    fn test_result_opaque() {
        let s = ResultOpaque::new(5);
        s.ok().unwrap().assert_integer(5);

        let error = ResultOpaque::new_failing_foo();
        assert!(matches!(error.err().unwrap(), ErrorEnum::Foo));

        let error = ResultOpaque::new_failing_bar();
        assert!(matches!(error.err().unwrap(), ErrorEnum::Bar));

        assert_eq!(ResultOpaque::new_failing_unit().err().unwrap(), ());

        let error = ResultOpaque::new_failing_struct(109);
        assert_eq!(error.err().unwrap().i, 109);

        let error = ResultOpaque::new_in_err(559);
        error.err().unwrap().assert_integer(559);

        let error = ResultOpaque::new_in_enum_err(881);
        error.err().unwrap().assert_integer(881);

        let error = MyStruct::fails_zst_result();
        assert!(matches!(error.err().unwrap(), MyZst{}));
    }

    #[test]
    fn test_error_struct() {
        let res = ErrorStruct::returns_result_option(true);
        assert!(res.as_ref().is_ok());
        let opt = res.unwrap();
        assert!(opt.is_some());
        assert_eq!(opt.unwrap().j, 125);

        let none = ErrorStruct::returns_result_option(false);
        assert!(none.as_ref().is_ok());
        let opt = none.unwrap();
        assert!(opt.is_none());
    }
}