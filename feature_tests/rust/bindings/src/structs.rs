#[allow(clippy::needless_lifetimes)]
#[diplomat_static_rust::bridge]
pub mod ffi {
    use diplomat_runtime::DiplomatStr16;
    use crate::imports::ffi::ImportedStruct;
    use std::fmt::Write;
    use std::sync::Mutex;
    pub struct Opaque(String);
    pub struct OpaqueMutexedString(Mutex<String>);
    pub struct Utf16Wrap(Vec<u16>);
    #[derive(Debug, PartialEq, Eq)]
    pub enum MyEnum {
        A = -2,
        B = -1,
        C = 0,
        #[diplomat::attr(auto, default)]
        D = 1,
        E = 2,
        F = 3,
    }
    #[derive(Debug, PartialEq, Eq)]
    pub enum ContiguousEnum {
        C = 0,
        D = 1,
        #[diplomat::attr(auto, default)]
        E = 2,
        F = 3,
    }
    pub enum MyOpaqueEnum {
        A(String),
        B(Utf16Wrap),
        C,
        D(i32, ImportedStruct),
    }
    pub enum DefaultEnum {
        A,
        B,
    }
    pub struct MyStruct {
        a: u8,
        b: bool,
        c: u8,
        d: u64,
        e: i32,
        f: DiplomatChar,
        g: MyEnum,
    }
    pub struct MyStructContainingAnOption {
        pub(crate) a: DiplomatOption<MyStruct>,
        pub(crate) b: DiplomatOption<DefaultEnum>,
    }
    pub struct MyZst;
    impl Opaque {
        pub fn new() -> Box<Opaque> {
            unsafe {}
        }
        pub fn try_from_utf8(input: &DiplomatStr) -> Option<Box<Self>> {
            unsafe {}
        }
        pub fn from_str(input: &str) -> Box<Self> {
            unsafe {}
        }
        pub fn get_debug_str(&self, write: &mut DiplomatWrite) {
            unsafe {}
        }
        pub fn assert_struct(&self, s: MyStruct) {
            unsafe {}
        }
        pub fn returns_usize() -> usize {
            unsafe {}
        }
        pub fn returns_imported() -> ImportedStruct {
            unsafe {}
        }
        pub fn cmp() -> core::cmp::Ordering {
            unsafe {}
        }
    }
    impl OpaqueMutexedString {
        pub fn from_usize(number: usize) -> Box<OpaqueMutexedString> {
            unsafe {}
        }
        pub fn change(&self, number: usize) {
            unsafe {}
        }
        pub fn borrow<'a>(&'a self) -> &'a OpaqueMutexedString {
            unsafe {}
        }
        pub fn borrow_other<'a>(
            other: &'a OpaqueMutexedString,
        ) -> &'a OpaqueMutexedString {
            unsafe {}
        }
        pub fn borrow_self_or_other<'a>(
            &'a self,
            other: &'a OpaqueMutexedString,
        ) -> &'a OpaqueMutexedString {
            unsafe {}
        }
        pub fn get_len_and_add(&self, other: usize) -> usize {
            unsafe {}
        }
        pub fn dummy_str<'a>(&'a self) -> &'a DiplomatStr {
            unsafe {}
        }
        pub fn wrapper<'a>(&'a self) -> Box<Utf16Wrap> {
            unsafe {}
        }
        pub fn to_unsigned_from_unsigned(&self, input: u16) -> u16 {
            unsafe {}
        }
    }
    impl Utf16Wrap {
        pub fn from_utf16(input: &DiplomatStr16) -> Box<Self> {
            unsafe {}
        }
        pub fn get_debug_str(&self, write: &mut DiplomatWrite) {
            unsafe {}
        }
        pub fn borrow_cont<'a>(&'a self) -> &'a DiplomatStr16 {
            unsafe {}
        }
    }
    impl MyEnum {
        pub fn into_value(self) -> i8 {
            unsafe {}
        }
        pub fn get_a() -> MyEnum {
            unsafe {}
        }
    }
    impl MyOpaqueEnum {
        pub fn new() -> Box<MyOpaqueEnum> {
            unsafe {}
        }
        pub fn to_string(&self, write: &mut DiplomatWrite) {
            unsafe {}
        }
    }
    impl DefaultEnum {
        pub fn new() -> DefaultEnum {
            unsafe {}
        }
    }
    impl MyStruct {
        pub fn new() -> MyStruct {
            unsafe {}
        }
        pub fn takes_mut(&mut self, o: &mut Self) {
            unsafe {}
        }
        pub fn takes_const(&self, o: &mut Self) {
            unsafe {}
        }
        pub fn into_a(self) -> u8 {
            unsafe {}
        }
        fn assert_value(&self) {
            unsafe {}
        }
        pub fn returns_zst_result() -> Result<(), MyZst> {
            unsafe {}
        }
        pub fn fails_zst_result() -> Result<(), MyZst> {
            unsafe {}
        }
    }
    impl MyStructContainingAnOption {
        pub fn new() -> Self {
            unsafe {}
        }
        pub fn filled() -> Self {
            unsafe {}
        }
    }
    #[derive(Default)]
    pub struct CyclicStructA {
        pub a: CyclicStructB,
    }
    #[derive(Default)]
    pub struct CyclicStructB {
        pub field: u8,
    }
    #[derive(Default)]
    pub struct CyclicStructC {
        pub a: CyclicStructA,
    }
    impl CyclicStructA {
        pub fn get_b() -> CyclicStructB {
            unsafe {}
        }
        pub fn cyclic_out(self, out: &mut DiplomatWrite) {
            unsafe {}
        }
        pub fn nested_slice(sl: &[CyclicStructA]) -> u8 {
            unsafe {}
        }
        pub fn double_cyclic_out(self, cyclic_struct_a: Self, out: &mut DiplomatWrite) {
            unsafe {}
        }
        pub fn getter_out(self, out: &mut DiplomatWrite) {
            unsafe {}
        }
    }
    impl CyclicStructB {
        pub fn get_a() -> CyclicStructA {
            unsafe {}
        }
        pub fn get_a_option() -> Option<CyclicStructA> {
            unsafe {}
        }
    }
    impl CyclicStructC {
        pub fn takes_nested_parameters(c: CyclicStructC) -> CyclicStructC {
            unsafe {}
        }
        pub fn cyclic_out(self, out: &mut DiplomatWrite) {
            unsafe {}
        }
    }
    /// Testing JS-specific layout/padding behavior
    pub struct ScalarPairWithPadding {
        pub first: u8,
        pub second: u32,
    }
    impl ScalarPairWithPadding {
        pub fn assert_value(self) {
            unsafe {}
        }
    }
    /// Testing JS-specific layout/padding behavior
    /// Also being used to test CPP backends taking structs with primitive values.
    pub struct BigStructWithStuff {
        pub first: u8,
        pub second: u16,
        pub third: u16,
        pub fourth: ScalarPairWithPadding,
        pub fifth: u8,
    }
    impl BigStructWithStuff {
        pub fn assert_value(self, extra_val: u16) {
            unsafe {}
        }
        pub fn assert_slice(slice: &[BigStructWithStuff], second_value: u16) {
            unsafe {}
        }
    }
    struct StructArithmetic {
        x: i32,
        y: i32,
    }
    impl StructArithmetic {
        #[allow(non_snake_case)]
        pub fn ORIGIN() -> Self {
            unsafe {}
        }
        pub fn set_origin(_new_origin: StructArithmetic) {
            unsafe {}
        }
        pub fn new(x: i32, y: i32) -> Self {
            unsafe {}
        }
        pub fn add(self, o: Self) -> Self {
            unsafe {}
        }
        pub fn sub(self, o: Self) -> Self {
            unsafe {}
        }
        pub fn mul(self, o: Self) -> Self {
            unsafe {}
        }
        pub fn div(self, o: Self) -> Self {
            unsafe {}
        }
    }
    pub struct StructWithSlices<'a> {
        pub first: DiplomatStrSlice<'a>,
        pub second: DiplomatSlice<'a, u16>,
    }
    impl<'a> StructWithSlices<'a> {
        pub fn return_last(self, w: &mut DiplomatWrite) {
            unsafe {}
        }
    }
    #[derive(Clone)]
    pub struct PrimitiveStruct {
        x: f32,
        a: bool,
        pub(crate) b: DiplomatChar,
        c: i64,
        d: isize,
        e: DiplomatByte,
    }
    impl PrimitiveStruct {
        pub fn mutable_slice(a: &mut [PrimitiveStruct]) {
            unsafe {}
        }
        pub fn mutable_ref(&mut self, a: &mut Self) {
            unsafe {}
        }
    }
    pub struct PrimitiveStructVec(Vec<PrimitiveStruct>);
    impl PrimitiveStructVec {
        pub fn new() -> Box<Self> {
            unsafe {}
        }
        pub fn push(&mut self, value: PrimitiveStruct) {
            unsafe {}
        }
        pub fn len(&self) -> usize {
            unsafe {}
        }
        pub fn as_slice<'a>(&'a self) -> &'a [PrimitiveStruct] {
            unsafe {}
        }
        pub fn as_slice_mut<'a>(&'a mut self) -> &'a mut [PrimitiveStruct] {
            unsafe {}
        }
        pub fn get(&self, idx: usize) -> PrimitiveStruct {
            unsafe {}
        }
        pub fn take_slice_from_other_namespace(
            _sl: &[crate::attrs::ffi::StructWithAttrs],
        ) {
            unsafe {}
        }
    }
}
#[allow(unused)]
fn test_transparent_convert_exists(s: &String) -> &ffi::Opaque {
    ffi::Opaque::transparent_convert(s)
}
