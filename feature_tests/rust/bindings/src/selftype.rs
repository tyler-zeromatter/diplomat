#[diplomat_static_rust::bridge(lib_name = "somelib")]
mod ffi {
    struct RefListParameter;
    struct RefList<'a>((&'a RefListParameter, Option<Box<Self>>));
    impl<'b> RefList<'b> {
        pub fn node(data: &'b RefListParameter) -> Box<Self> {
            unsafe { RefList_node(data) }
        }
    }
}
