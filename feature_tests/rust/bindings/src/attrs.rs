#[diplomat_static_rust::bridge]
pub mod ffi {
    #[diplomat::macro_rules]
    macro_rules! impl_mac {
        ($arg1:ident, $arg2:ident, $arg3:block) => {
            pub fn $arg1 () -> i32 { $arg3 } pub fn $arg2 () -> i32 { println!("Test"); 0
            }
        };
    }
    #[diplomat::macro_rules]
    macro_rules! create_vec {
        ($vec_name:ident contains "hello"; [$ty:ident]) => {
            #[diplomat::opaque] pub struct $vec_name (Vec <$ty >); impl $vec_name {
            #[diplomat::attr(auto, constructor)] pub fn new() -> Box <$vec_name > {
            println!("{}", stringify!($vec_name)); Box::new(Self(Vec::new())) }
            #[diplomat::attr(auto, getter)] pub fn len(& self) -> usize { self.0.len() }
            #[diplomat::attr(auto, indexer)] pub fn get(& self, idx : usize) -> Option
            <$ty > { self.0.get(idx).cloned() } pub fn push(& mut self, value : $ty) {
            self.0.push(value) } }
        };
    }
    create_vec!(VectorTest contains "hello"; [f64]);
    #[derive(Clone)]
    pub struct AttrOpaque1;
    impl AttrOpaque1 {
        pub fn new() -> Box<AttrOpaque1> {
            unsafe {}
        }
        pub fn test_namespaced_callback(_t: impl Fn() -> Result<(), ()>) {
            unsafe {}
        }
        impl_mac!(mac_test, hello, { println!("Hello world!"); 10 });
        pub fn method(&self) -> u8 {
            unsafe {}
        }
        pub fn abirenamed(&self) -> u8 {
            unsafe {}
        }
        pub fn method_disabled(&self) {
            unsafe {}
        }
        pub fn use_unnamespaced(&self, _un: &Unnamespaced) {
            unsafe {}
        }
        pub fn use_namespaced(&self, _n: AttrEnum) {
            unsafe {}
        }
    }
    pub struct AttrOpaque2;
    pub enum AttrEnum {
        A,
        B,
        #[diplomat::attr(*, rename = "Renamed")]
        C,
    }
    pub struct Unnamespaced;
    impl Unnamespaced {
        pub fn make(_e: AttrEnum) -> Box<Self> {
            unsafe {}
        }
        pub fn use_namespaced(&self, _n: &AttrOpaque1) {
            unsafe {}
        }
    }
    pub struct Nested;
    pub struct Nested2;
    pub struct Comparable(u8);
    impl Comparable {
        pub fn new(int: u8) -> Box<Self> {
            unsafe {}
        }
        pub fn cmp(&self, other: &Comparable) -> core::cmp::Ordering {
            unsafe {}
        }
    }
    pub struct MyIndexer(Vec<String>);
    pub struct MyIterable(Vec<u8>);
    impl MyIterable {
        pub fn new(x: &[u8]) -> Box<Self> {
            unsafe {}
        }
        pub fn iter<'a>(&'a self) -> Box<MyIterator<'a>> {
            unsafe {}
        }
        pub fn len(&self) -> usize {
            unsafe {}
        }
    }
    pub struct MyIterator<'a>(std::slice::Iter<'a, u8>);
    impl<'a> MyIterator<'a> {
        pub fn next(&mut self) -> Option<u8> {
            unsafe {}
        }
    }
    impl MyIndexer {
        pub fn get<'a>(&'a self, i: usize) -> Option<&'a DiplomatStr> {
            unsafe {}
        }
    }
    struct OpaqueIterable(Vec<AttrOpaque1>);
    impl OpaqueIterable {
        pub fn iter<'a>(&'a self) -> Box<OpaqueIterator<'a>> {
            unsafe {}
        }
    }
    struct OpaqueIterator<'a>(Box<dyn Iterator<Item = AttrOpaque1> + 'a>);
    impl<'a> OpaqueIterator<'a> {
        pub fn next(&'a mut self) -> Option<Box<AttrOpaque1>> {
            unsafe {}
        }
    }
    pub(crate) struct OpaqueArithmetic {
        x: i32,
        y: i32,
    }
    impl OpaqueArithmetic {
        pub fn make(x: i32, y: i32) -> Box<Self> {
            unsafe {}
        }
        pub fn make_overload(x: f32, y: f32) -> Box<Self> {
            unsafe {}
        }
        pub fn x(&self) -> i32 {
            unsafe {}
        }
        pub fn y(&self) -> i32 {
            unsafe {}
        }
        pub fn add(&self, o: &Self) -> Box<Self> {
            unsafe {}
        }
        pub fn sub(&self, o: &Self) -> Box<Self> {
            unsafe {}
        }
        pub fn mul(&self, o: &Self) -> Box<Self> {
            unsafe {}
        }
        pub fn div(&self, o: &Self) -> Box<Self> {
            unsafe {}
        }
        pub fn addassign(&mut self, o: &Self) {
            unsafe {}
        }
        pub fn subassign(&mut self, o: &Self) {
            unsafe {}
        }
        pub fn mulassign(&mut self, o: &Self) {
            unsafe {}
        }
        pub fn divassign(&mut self, o: &Self) {
            unsafe {}
        }
    }
    pub struct StructWithAttrs {
        a: bool,
        b: u32,
    }
    impl StructWithAttrs {
        pub fn new_fallible(a: bool, b: u32) -> Result<StructWithAttrs, ()> {
            unsafe {}
        }
        pub fn c(self) -> u32 {
            unsafe {}
        }
    }
    #[diplomat::macro_rules]
    macro_rules! macro_frag_spec_test {
        (
            BLOCK $b:block [EXPR $e:expr, IDENT $i:ident] LT $lt:lifetime literal
            $l:literal <=> $m:meta $p:path; $t:tt $ty:ty, $vis:vis, $it:item
        ) => {
            struct $i { a : usize, } $it use $p; impl $i {
            #[allow(clippy::extra_unused_lifetimes)] $vis fn test_func <$lt > (w : & mut
            DiplomatWrite) -> usize { let a = $e; write!(w, $l) .unwrap(); a } #[$m] $vis
            fn test_meta() -> $i { $b $i { a : 0 } } } #[diplomat::opaque] struct
            TestOpaque($ty); impl TestOpaque $t
        };
    }
    macro_frag_spec_test! {
        BLOCK { println!("Hello world"); } [EXPR 0, IDENT TestMacroStruct] LT 'a literal
        "Testing" <=> diplomat::attr(auto, constructor) std::fmt::Write; { fn hello() {}
        } f64, pub, const IT : usize = 0;
    }
}
