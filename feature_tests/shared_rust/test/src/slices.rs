#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{Float64Vec, MyString, OptionString};

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

    // TODO: Need to make sure &[String] works:
    // #[test]
    // fn test_string_list() {
    //     let out = MyString::new_from_first(vec!["This", "is", "a", "test"]);
    //     assert_eq!(out.get_str(), "This")
    // }

    #[test]
    fn test_float64_vec() {
        let data = vec![1, 2, 3, 4, 5];
        let vec = Float64Vec::new_isize(&data);
        let converted : Vec<f64> = data.iter().map(|&i| { i as f64 }).collect();
        assert_eq!(&converted, vec.borrow());
    }

    #[test]
    fn test_string_owned() {
        let owned : Box<[u8]> = Box::new([b'T', b'e', b's', b't']);
        let str = MyString::new_owned(owned);
        assert_eq!(str.borrow(), "Test".as_bytes());
    }
}