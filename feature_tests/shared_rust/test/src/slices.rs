#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{OptionString, MyString};

    #[test]
    fn test_string_opt() {
        let string = format!("This is a test");
        let opt = OptionString::new(string.as_bytes()).unwrap();
        assert_eq!(opt.write().unwrap(), string)
    }

    #[test]
    fn test_slices() {
        let str = format!("Testing 123");
        let mut my_string_unchecked = MyString::new(str.as_bytes());
        {
            let bytes = my_string_unchecked.borrow();
            assert_eq!(str.as_bytes(), bytes);
        }

        assert_eq!(my_string_unchecked.get_str(), str);
        
        let new_str = format!("This is another test");
        my_string_unchecked.set_str(new_str.as_bytes());
        assert_eq!(my_string_unchecked.get_str(), new_str);
    }
}