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

    #[test]
    fn test_string_list() {
        let out = MyString::new_from_first(&vec!["This".as_bytes(), "is".as_bytes(), "a".as_bytes(), "test".as_bytes()]);
        assert_eq!(out.get_str(), "This")
    }

    #[test]
    fn test_mutable_string() {
        let mut out = MyString::new_unsafe("some string");
        assert_eq!(out.borrow(), [115, 111, 109, 101, 32, 115, 116, 114, 105, 110, 103]);
        out.set_str(&[83, 111, 109, 101, 32, 115, 116, 114, 105, 110, 103]);
        assert_eq!(out.get_str(), "Some string");
    }

    #[test]
    fn test_statics() {
        let out = MyString::get_static_str();
        assert_eq!(out, "hello");
    }

    #[test]
    fn test_float64_vec() {
        let data = vec![1, 2, 3, 4, 5];
        let mut vec = Float64Vec::new_isize(&data);
        let converted : Vec<f64> = data.iter().map(|&i| { i as f64 }).collect();
        assert_eq!(&converted, vec.borrow());

        assert_eq!(vec.to_string(), "[1.0, 2.0, 3.0, 4.0, 5.0]");
        assert_eq!(vec.get(0), Some(1.0));
        vec.set_value(&[5.0, 6.0, 7.0, 8.0]);
        assert_eq!(*vec.as_slice(), [5.0, 6.0, 7.0, 8.0])
    }
}