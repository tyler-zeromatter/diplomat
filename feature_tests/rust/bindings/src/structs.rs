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
            unsafe { StructArithmetic_new() }
        }
        pub fn try_from_utf8(input: &DiplomatStr) -> Option<Box<Self>> {
            unsafe { Opaque_try_from_utf8(input) }
        }
        pub fn from_str(input: &str) -> Box<Self> {
            unsafe { Opaque_from_str(input) }
        }
        pub fn get_debug_str(&self, write: &mut DiplomatWrite) {
            unsafe { Utf16Wrap_get_debug_str(self, write) }
        }
        pub fn assert_struct(&self, s: MyStruct) {
            unsafe { Opaque_assert_struct(self, s) }
        }
        pub fn returns_usize() -> usize {
            unsafe { Opaque_returns_usize() }
        }
        pub fn returns_imported() -> ImportedStruct {
            unsafe { Opaque_returns_imported() }
        }
        pub fn cmp() -> core::cmp::Ordering {
            unsafe { Opaque_cmp() }
        }
    }
    impl OpaqueMutexedString {
        pub fn from_usize(number: usize) -> Box<OpaqueMutexedString> {
            unsafe { OpaqueMutexedString_from_usize(number) }
        }
        pub fn change(&self, number: usize) {
            unsafe { OpaqueMutexedString_change(self, number) }
        }
        pub fn borrow<'a>(&'a self) -> &'a OpaqueMutexedString {
            unsafe { OpaqueMutexedString_borrow(self) }
        }
        pub fn borrow_other<'a>(
            other: &'a OpaqueMutexedString,
        ) -> &'a OpaqueMutexedString {
            unsafe { OpaqueMutexedString_borrow_other(other) }
        }
        pub fn borrow_self_or_other<'a>(
            &'a self,
            other: &'a OpaqueMutexedString,
        ) -> &'a OpaqueMutexedString {
            unsafe { OpaqueMutexedString_borrow_self_or_other(self, other) }
        }
        pub fn get_len_and_add(&self, other: usize) -> usize {
            unsafe { OpaqueMutexedString_get_len_and_add(self, other) }
        }
        pub fn dummy_str<'a>(&'a self) -> &'a DiplomatStr {
            unsafe { OpaqueMutexedString_dummy_str(self) }
        }
        pub fn wrapper<'a>(&'a self) -> Box<Utf16Wrap> {
            unsafe { OpaqueMutexedString_wrapper(self) }
        }
        pub fn to_unsigned_from_unsigned(&self, input: u16) -> u16 {
            unsafe { OpaqueMutexedString_to_unsigned_from_unsigned(self, input) }
        }
    }
    impl Utf16Wrap {
        pub fn from_utf16(input: &DiplomatStr16) -> Box<Self> {
            unsafe { Utf16Wrap_from_utf16(input) }
        }
        pub fn get_debug_str(&self, write: &mut DiplomatWrite) {
            unsafe { Utf16Wrap_get_debug_str(self, write) }
        }
        pub fn borrow_cont<'a>(&'a self) -> &'a DiplomatStr16 {
            unsafe { Utf16Wrap_borrow_cont(self) }
        }
    }
    impl MyEnum {
        pub fn into_value(self) -> i8 {
            unsafe { MyEnum_into_value(self) }
        }
        pub fn get_a() -> MyEnum {
            unsafe { MyEnum_get_a() }
        }
    }
    impl MyOpaqueEnum {
        pub fn new() -> Box<MyOpaqueEnum> {
            unsafe { StructArithmetic_new() }
        }
        pub fn to_string(&self, write: &mut DiplomatWrite) {
            unsafe { MyOpaqueEnum_to_string(self, write) }
        }
    }
    impl DefaultEnum {
        pub fn new() -> DefaultEnum {
            unsafe { StructArithmetic_new() }
        }
    }
    impl MyStruct {
        pub fn new() -> MyStruct {
            unsafe { StructArithmetic_new() }
        }
        pub fn takes_mut(&mut self, o: &mut Self) {
            unsafe { MyStruct_takes_mut(self, o) }
        }
        pub fn takes_const(&self, o: &mut Self) {
            unsafe { MyStruct_takes_const(self, o) }
        }
        pub fn into_a(self) -> u8 {
            unsafe { MyStruct_into_a(self) }
        }
        fn assert_value(&self) {
            unsafe { ScalarPairWithPadding_assert_value(self) }
        }
        pub fn returns_zst_result() -> Result<(), MyZst> {
            unsafe { MyStruct_returns_zst_result() }
        }
        pub fn fails_zst_result() -> Result<(), MyZst> {
            unsafe { MyStruct_fails_zst_result() }
        }
    }
    impl MyStructContainingAnOption {
        pub fn new() -> Self {
            unsafe { StructArithmetic_new() }
        }
        pub fn filled() -> Self {
            unsafe { MyStructContainingAnOption_filled() }
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
            unsafe { CyclicStructA_get_b() }
        }
        pub fn cyclic_out(self, out: &mut DiplomatWrite) {
            unsafe { CyclicStructC_cyclic_out(self, out) }
        }
        pub fn nested_slice(sl: &[CyclicStructA]) -> u8 {
            unsafe { CyclicStructA_nested_slice(sl) }
        }
        pub fn double_cyclic_out(self, cyclic_struct_a: Self, out: &mut DiplomatWrite) {
            unsafe { CyclicStructA_double_cyclic_out(self, cyclic_struct_a, out) }
        }
        pub fn getter_out(self, out: &mut DiplomatWrite) {
            unsafe { CyclicStructA_getter_out(self, out) }
        }
    }
    impl CyclicStructB {
        pub fn get_a() -> CyclicStructA {
            unsafe { MyEnum_get_a() }
        }
        pub fn get_a_option() -> Option<CyclicStructA> {
            unsafe { CyclicStructB_get_a_option() }
        }
    }
    impl CyclicStructC {
        pub fn takes_nested_parameters(c: CyclicStructC) -> CyclicStructC {
            unsafe { CyclicStructC_takes_nested_parameters(c) }
        }
        pub fn cyclic_out(self, out: &mut DiplomatWrite) {
            unsafe { CyclicStructC_cyclic_out(self, out) }
        }
    }
    /// Testing JS-specific layout/padding behavior
    pub struct ScalarPairWithPadding {
        pub first: u8,
        pub second: u32,
    }
    impl ScalarPairWithPadding {
        pub fn assert_value(self) {
            unsafe { ScalarPairWithPadding_assert_value(self) }
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
            unsafe { ScalarPairWithPadding_assert_value(self, extra_val) }
        }
        pub fn assert_slice(slice: &[BigStructWithStuff], second_value: u16) {
            unsafe { BigStructWithStuff_assert_slice(slice, second_value) }
        }
    }
    struct StructArithmetic {
        x: i32,
        y: i32,
    }
    impl StructArithmetic {
        #[allow(non_snake_case)]
        pub fn ORIGIN() -> Self {
            unsafe { StructArithmetic_ORIGIN() }
        }
        pub fn set_origin(_new_origin: StructArithmetic) {
            unsafe { StructArithmetic_set_origin(_new_origin) }
        }
        pub fn new(x: i32, y: i32) -> Self {
            unsafe { StructArithmetic_new(x, y) }
        }
        pub fn add(self, o: Self) -> Self {
            unsafe { StructArithmetic_add(self, o) }
        }
        pub fn sub(self, o: Self) -> Self {
            unsafe { StructArithmetic_sub(self, o) }
        }
        pub fn mul(self, o: Self) -> Self {
            unsafe { StructArithmetic_mul(self, o) }
        }
        pub fn div(self, o: Self) -> Self {
            unsafe { StructArithmetic_div(self, o) }
        }
    }
    pub struct StructWithSlices<'a> {
        pub first: DiplomatStrSlice<'a>,
        pub second: DiplomatSlice<'a, u16>,
    }
    impl<'a> StructWithSlices<'a> {
        pub fn return_last(self, w: &mut DiplomatWrite) {
            unsafe { StructWithSlices_return_last(self, w) }
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
            unsafe { PrimitiveStruct_mutable_slice(a) }
        }
        pub fn mutable_ref(&mut self, a: &mut Self) {
            unsafe { PrimitiveStruct_mutable_ref(self, a) }
        }
    }
    pub struct PrimitiveStructVec(Vec<PrimitiveStruct>);
    impl PrimitiveStructVec {
        pub fn new() -> Box<Self> {
            unsafe { StructArithmetic_new() }
        }
        pub fn push(&mut self, value: PrimitiveStruct) {
            unsafe { PrimitiveStructVec_push(self, value) }
        }
        pub fn len(&self) -> usize {
            unsafe { PrimitiveStructVec_len(self) }
        }
        pub fn as_slice<'a>(&'a self) -> &'a [PrimitiveStruct] {
            unsafe { PrimitiveStructVec_as_slice(self) }
        }
        pub fn as_slice_mut<'a>(&'a mut self) -> &'a mut [PrimitiveStruct] {
            unsafe { PrimitiveStructVec_as_slice_mut(self) }
        }
        pub fn get(&self, idx: usize) -> PrimitiveStruct {
            unsafe { PrimitiveStructVec_get(self, idx) }
        }
        pub fn take_slice_from_other_namespace(
            _sl: &[crate::attrs::ffi::StructWithAttrs],
        ) {
            unsafe { PrimitiveStructVec_take_slice_from_other_namespace(_sl) }
        }
    }
}
#[allow(unused)]
fn test_transparent_convert_exists(s: &String) -> &ffi::Opaque {
    ffi::Opaque::transparent_convert(s)
}
