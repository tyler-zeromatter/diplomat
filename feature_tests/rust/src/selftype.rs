#[diplomat::bridge]
mod ffi {
    #[test]
    struct RefListParameter;
    #[test]
    struct RefList<'a>((&'a RefListParameter, Option<Box<Self>>));
    impl<'b> RefList<'b> {
        #[test]
        pub fn node(data: &'b RefListParameter) -> Box<Self> {
            Box::new(RefList((data, None)))
        }
    }
}
