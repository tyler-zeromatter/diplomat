#[diplomat::bridge]
#[diplomat::abi_rename = "namespace_{0}"]
#[diplomat::attr(not(any(c, kotlin)), rename = "Renamed{0}")]
#[diplomat::attr(auto, namespace = "ns")]
pub mod ffi {
    #[test]
    macro_rules! impl_mac {
        ($arg1:ident, $arg2:ident, $arg3:block) => {
            pub fn $arg1 () -> i32 { $arg3 } pub fn $arg2 () -> i32 { println!("Test"); 0
            }
        };
    }
    #[test]
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
    #[test]
    #[test]
    #[test]
    #[test]
    pub struct AttrOpaque1;
    impl AttrOpaque1 {
        #[test]
        #[test]
        pub fn new() -> Box<AttrOpaque1> {
            Box::new(AttrOpaque1)
        }
        #[test]
        pub fn test_namespaced_callback(_t: impl Fn() -> Result<(), ()>) {
            todo!()
        }
        impl_mac!(mac_test, hello, { println!("Hello world!"); 10 });
        #[test]
        #[test]
        pub fn method(&self) -> u8 {
            77
        }
        #[test]
        #[test]
        pub fn abirenamed(&self) -> u8 {
            123
        }
        #[test]
        pub fn method_disabled(&self) {
            println!("disabled in hir");
        }
        pub fn use_unnamespaced(&self, _un: &Unnamespaced) {}
        pub fn use_namespaced(&self, _n: AttrEnum) {}
    }
    #[test]
    pub struct AttrOpaque2;
    pub enum AttrEnum {
        A,
        B,
        #[test]
        C,
    }
    #[test]
    #[test]
    #[test]
    pub struct Unnamespaced;
    impl Unnamespaced {
        #[test]
        pub fn make(_e: AttrEnum) -> Box<Self> {
            Box::new(Self)
        }
        pub fn use_namespaced(&self, _n: &AttrOpaque1) {}
    }
    #[test]
    #[test]
    #[test]
    pub struct Nested;
    #[test]
    #[test]
    #[test]
    pub struct Nested2;
    #[test]
    #[test]
    pub struct Comparable(u8);
    impl Comparable {
        pub fn new(int: u8) -> Box<Self> {
            Box::new(Self(int))
        }
        #[test]
        pub fn cmp(&self, other: &Comparable) -> core::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }
    #[test]
    #[test]
    pub struct MyIndexer(Vec<String>);
    #[test]
    #[test]
    pub struct MyIterable(Vec<u8>);
    impl MyIterable {
        #[test]
        pub fn new(x: &[u8]) -> Box<Self> {
            Box::new(Self(x.into()))
        }
        #[test]
        pub fn iter<'a>(&'a self) -> Box<MyIterator<'a>> {
            Box::new(MyIterator(self.0.iter()))
        }
        #[test]
        #[test]
        pub fn len(&self) -> usize {
            self.0.len()
        }
    }
    #[test]
    #[test]
    pub struct MyIterator<'a>(std::slice::Iter<'a, u8>);
    impl<'a> MyIterator<'a> {
        #[test]
        pub fn next(&mut self) -> Option<u8> {
            self.0.next().copied()
        }
    }
    impl MyIndexer {
        #[test]
        pub fn get<'a>(&'a self, i: usize) -> Option<&'a DiplomatStr> {
            self.0.get(i).as_ref().map(|string| string.as_bytes())
        }
    }
    #[test]
    #[test]
    struct OpaqueIterable(Vec<AttrOpaque1>);
    impl OpaqueIterable {
        #[test]
        pub fn iter<'a>(&'a self) -> Box<OpaqueIterator<'a>> {
            Box::new(OpaqueIterator(Box::new(self.0.iter().cloned())))
        }
    }
    #[test]
    #[test]
    struct OpaqueIterator<'a>(Box<dyn Iterator<Item = AttrOpaque1> + 'a>);
    impl<'a> OpaqueIterator<'a> {
        #[test]
        pub fn next(&'a mut self) -> Option<Box<AttrOpaque1>> {
            self.0.next().map(Box::new)
        }
    }
    #[test]
    #[test]
    pub(crate) struct OpaqueArithmetic {
        x: i32,
        y: i32,
    }
    impl OpaqueArithmetic {
        pub fn make(x: i32, y: i32) -> Box<Self> {
            Box::new(Self { x, y })
        }
        #[test]
        pub fn make_overload(x: f32, y: f32) -> Box<Self> {
            Box::new(Self {
                x: (x as i32) + 2,
                y: y as i32,
            })
        }
        pub fn x(&self) -> i32 {
            self.x
        }
        pub fn y(&self) -> i32 {
            self.y
        }
        #[test]
        pub fn add(&self, o: &Self) -> Box<Self> {
            Box::new(Self {
                x: self.x + o.x,
                y: self.y + o.y,
            })
        }
        #[test]
        pub fn sub(&self, o: &Self) -> Box<Self> {
            Box::new(Self {
                x: self.x - o.x,
                y: self.y - o.y,
            })
        }
        #[test]
        pub fn mul(&self, o: &Self) -> Box<Self> {
            Box::new(Self {
                x: self.x * o.x,
                y: self.y * o.y,
            })
        }
        #[test]
        pub fn div(&self, o: &Self) -> Box<Self> {
            Box::new(Self {
                x: self.x / o.x,
                y: self.y / o.y,
            })
        }
        #[test]
        pub fn addassign(&mut self, o: &Self) {
            self.x += o.x;
            self.y += o.y;
        }
        #[test]
        pub fn subassign(&mut self, o: &Self) {
            self.x -= o.x;
            self.y -= o.y;
        }
        #[test]
        pub fn mulassign(&mut self, o: &Self) {
            self.x *= o.x;
            self.y *= o.y;
        }
        #[test]
        pub fn divassign(&mut self, o: &Self) {
            self.x /= o.x;
            self.y /= o.y;
        }
    }
    #[test]
    pub struct StructWithAttrs {
        a: bool,
        b: u32,
    }
    impl StructWithAttrs {
        #[test]
        #[test]
        pub fn new_fallible(a: bool, b: u32) -> Result<StructWithAttrs, ()> {
            if a { Ok(Self { a, b }) } else { Err(()) }
        }
        #[test]
        pub fn c(self) -> u32 {
            5
        }
    }
    #[test]
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
