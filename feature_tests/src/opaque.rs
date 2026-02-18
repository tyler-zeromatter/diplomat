#[diplomat::bridge]
mod ffi {
    use std::sync::Arc;


    #[diplomat::opaque(wrapper=Box<T>)]
    pub struct OpaqueAllocGlobal(String);

    impl OpaqueAllocGlobal {
        #[diplomat::attr(auto, constructor)]
        pub fn new() -> Box<Self> {
            Box::new(Self("Hello world".into()))
        }
    }
    
    #[diplomat::opaque(wrapper=Arc<T>)]
    pub struct OpaqueAllocArc(String);
    impl OpaqueAllocArc {
        #[diplomat::attr(auto, constructor)]
        pub fn new() -> Arc<Self> {
            
        }
    }
}