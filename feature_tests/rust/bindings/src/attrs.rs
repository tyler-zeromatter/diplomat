#[diplomat_static_rust::bridge(lib_name = "somelib")]
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
            unsafe { namespace_VectorTest_new() }
        }
        pub fn test_namespaced_callback(_t: impl Fn() -> Result<(), ()>) {
            unsafe { namespace_AttrOpaque1_test_namespaced_callback(_t) }
        }
        impl_mac!(mac_test, hello, { println!("Hello world!"); 10 });
        pub fn method(&self) -> u8 {
            unsafe { namespace_AttrOpaque1_method(self) }
        }
        pub fn abirenamed(&self) -> u8 {
            unsafe { renamed_on_abi_only(self) }
        }
        pub fn method_disabled(&self) {
            unsafe { namespace_AttrOpaque1_method_disabled(self) }
        }
        pub fn use_unnamespaced(&self, _un: &Unnamespaced) {
            unsafe { namespace_AttrOpaque1_use_unnamespaced(self, _un) }
        }
        pub fn use_namespaced(&self, _n: AttrEnum) {
            unsafe { namespace_Unnamespaced_use_namespaced(self, _n) }
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
            unsafe { namespace_Unnamespaced_make(_e) }
        }
        pub fn use_namespaced(&self, _n: &AttrOpaque1) {
            unsafe { namespace_Unnamespaced_use_namespaced(self, _n) }
        }
    }
    pub struct Nested;
    pub struct Nested2;
    pub struct Comparable(u8);
    impl Comparable {
        pub fn new(int: u8) -> Box<Self> {
            unsafe { namespace_VectorTest_new(int) }
        }
        pub fn cmp(&self, other: &Comparable) -> core::cmp::Ordering {
            unsafe { namespace_Comparable_cmp(self, other) }
        }
    }
    pub struct MyIndexer(Vec<String>);
    pub struct MyIterable(Vec<u8>);
    impl MyIterable {
        pub fn new(x: &[u8]) -> Box<Self> {
            unsafe { namespace_VectorTest_new(x) }
        }
        pub fn iter<'a>(&'a self) -> Box<MyIterator<'a>> {
            unsafe { namespace_OpaqueIterable_iter(self) }
        }
        pub fn len(&self) -> usize {
            unsafe { namespace_VectorTest_len(self) }
        }
    }
    pub struct MyIterator<'a>(std::slice::Iter<'a, u8>);
    impl<'a> MyIterator<'a> {
        pub fn next(&mut self) -> Option<u8> {
            unsafe { namespace_OpaqueIterator_next(self) }
        }
    }
    impl MyIndexer {
        pub fn get<'a>(&'a self, i: usize) -> Option<&'a DiplomatStr> {
            unsafe { namespace_VectorTest_get(self, i) }
        }
    }
    struct OpaqueIterable(Vec<AttrOpaque1>);
    impl OpaqueIterable {
        pub fn iter<'a>(&'a self) -> Box<OpaqueIterator<'a>> {
            unsafe { namespace_OpaqueIterable_iter(self) }
        }
    }
    struct OpaqueIterator<'a>(Box<dyn Iterator<Item = AttrOpaque1> + 'a>);
    impl<'a> OpaqueIterator<'a> {
        pub fn next(&'a mut self) -> Option<Box<AttrOpaque1>> {
            unsafe { namespace_OpaqueIterator_next(self) }
        }
    }
    pub(crate) struct OpaqueArithmetic {
        x: i32,
        y: i32,
    }
    impl OpaqueArithmetic {
        pub fn make(x: i32, y: i32) -> Box<Self> {
            unsafe { namespace_Unnamespaced_make(x, y) }
        }
        pub fn make_overload(x: f32, y: f32) -> Box<Self> {
            unsafe { namespace_OpaqueArithmetic_make_overload(x, y) }
        }
        pub fn x(&self) -> i32 {
            unsafe { namespace_OpaqueArithmetic_x(self) }
        }
        pub fn y(&self) -> i32 {
            unsafe { namespace_OpaqueArithmetic_y(self) }
        }
        pub fn add(&self, o: &Self) -> Box<Self> {
            unsafe { namespace_OpaqueArithmetic_add(self, o) }
        }
        pub fn sub(&self, o: &Self) -> Box<Self> {
            unsafe { namespace_OpaqueArithmetic_sub(self, o) }
        }
        pub fn mul(&self, o: &Self) -> Box<Self> {
            unsafe { namespace_OpaqueArithmetic_mul(self, o) }
        }
        pub fn div(&self, o: &Self) -> Box<Self> {
            unsafe { namespace_OpaqueArithmetic_div(self, o) }
        }
        pub fn addassign(&mut self, o: &Self) {
            unsafe { namespace_OpaqueArithmetic_addassign(self, o) }
        }
        pub fn subassign(&mut self, o: &Self) {
            unsafe { namespace_OpaqueArithmetic_subassign(self, o) }
        }
        pub fn mulassign(&mut self, o: &Self) {
            unsafe { namespace_OpaqueArithmetic_mulassign(self, o) }
        }
        pub fn divassign(&mut self, o: &Self) {
            unsafe { namespace_OpaqueArithmetic_divassign(self, o) }
        }
    }
    pub struct StructWithAttrs {
        a: bool,
        b: u32,
    }
    impl StructWithAttrs {
        pub fn new_fallible(a: bool, b: u32) -> Result<StructWithAttrs, ()> {
            unsafe { namespace_StructWithAttrs_new_fallible(a, b) }
        }
        pub fn c(self) -> u32 {
            unsafe { namespace_StructWithAttrs_c(self) }
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
