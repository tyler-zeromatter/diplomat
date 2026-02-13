#[diplomat::bridge]
mod ffi {

    #[diplomat::opaque(wrapper=Box<T, Global>)]
    pub struct OpaqueAllocGlobal(String);

    impl OpaqueAllocGlobal {
        #[diplomat::attr(auto, constructor)]
        pub fn new() -> Box<Self> {
            Box::new(Self("Hello world".into()))
        }
    }

    #[cfg(feature="allocator_trait")]
    #[diplomat::cfg(feature=allocator_trait)]
    mod custom_allocators {
        #[diplomat::opaque(wrapper=Box<T, CustomAlloc>)]
        pub struct CustomBoxAlloc;
    }
}