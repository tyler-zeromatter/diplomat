#[cfg(test)]
mod tests {
    #[test]
    fn test_call() {
        let a = feature_tests_static_rust::imports::ffi::ImportedStruct::small_test();
        a.assert_eq(5);
    }

}