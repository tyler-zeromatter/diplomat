#[diplomat_static_rust::bridge]
pub mod ffi {
    use std::fmt::Write;
    use diplomat_runtime::DiplomatStr16;
    pub struct Foo<'a>(&'a DiplomatStr);
    pub struct Bar<'b, 'a: 'b>(&'b Foo<'a>);
    pub struct BorrowedFields<'a> {
        a: DiplomatStr16Slice<'a>,
        b: DiplomatStrSlice<'a>,
        c: DiplomatUtf8StrSlice<'a>,
    }
    pub struct BorrowedFieldsWithBounds<'a, 'b: 'a, 'c: 'b> {
        field_a: DiplomatStr16Slice<'a>,
        field_b: DiplomatStrSlice<'b>,
        field_c: DiplomatUtf8StrSlice<'c>,
    }
    pub struct BorrowedFieldsReturning<'a> {
        bytes: DiplomatStrSlice<'a>,
    }
    impl<'a> Foo<'a> {
        pub fn new(x: &'a DiplomatStr) -> Box<Self> {
            unsafe { Foo_new(x) }
        }
        pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> {
            unsafe { Foo_get_bar(self) }
        }
        pub fn new_static(x: &'static DiplomatStr) -> Box<Self> {
            unsafe { Foo_new_static(x) }
        }
        pub fn as_returning(&self) -> BorrowedFieldsReturning<'a> {
            unsafe { Foo_as_returning(self) }
        }
        pub fn extract_from_fields(fields: BorrowedFields<'a>) -> Box<Self> {
            unsafe { Foo_extract_from_fields(fields) }
        }
        /// Test that the extraction logic correctly pins the right fields
        pub fn extract_from_bounds<'x, 'y: 'x + 'a, 'z: 'x + 'y>(
            bounds: BorrowedFieldsWithBounds<'x, 'y, 'z>,
            another_string: &'a DiplomatStr,
        ) -> Box<Self> {
            unsafe { Foo_extract_from_bounds(bounds, another_string) }
        }
    }
    impl<'x> BorrowedFields<'x> {
        pub fn from_bar_and_strings(
            bar: &'x Bar<'x, 'x>,
            dstr16: &'x DiplomatStr16,
            utf8_str: &'x str,
        ) -> Self {
            unsafe { BorrowedFields_from_bar_and_strings(bar, dstr16, utf8_str) }
        }
    }
    impl<'x, 'y: 'x, 'z: 'y> BorrowedFieldsWithBounds<'x, 'y, 'z> {
        pub fn from_foo_and_strings(
            foo: &'x Foo<'y>,
            dstr16_x: &'x DiplomatStr16,
            utf8_str_z: &'z str,
        ) -> Self {
            unsafe {
                BorrowedFieldsWithBounds_from_foo_and_strings(foo, dstr16_x, utf8_str_z)
            }
        }
    }
    pub struct NestedBorrowedFields<'x, 'y: 'x, 'z> {
        fields: BorrowedFields<'x>,
        bounds: BorrowedFieldsWithBounds<'x, 'y, 'y>,
        bounds2: BorrowedFieldsWithBounds<'z, 'z, 'z>,
    }
    impl<'x, 'y: 'x, 'z> NestedBorrowedFields<'x, 'y, 'z> {
        pub fn from_bar_and_foo_and_strings(
            bar: &'x Bar<'x, 'y>,
            foo: &'z Foo<'z>,
            dstr16_x: &'x DiplomatStr16,
            dstr16_z: &'z DiplomatStr16,
            utf8_str_y: &'y str,
            utf8_str_z: &'z str,
        ) -> Self {
            unsafe {
                NestedBorrowedFields_from_bar_and_foo_and_strings(
                    bar,
                    foo,
                    dstr16_x,
                    dstr16_z,
                    utf8_str_y,
                    utf8_str_z,
                )
            }
        }
    }
    impl<'b, 'a: 'b> Bar<'b, 'a> {
        pub fn foo(&'b self) -> &'b Foo<'a> {
            unsafe { Bar_foo(self) }
        }
    }
    #[derive(Copy, Clone)]
    pub struct One<'a>(super::One<'a>);
    #[derive(Copy, Clone)]
    pub struct Two<'a, 'b>(super::Two<'a, 'b>);
    impl<'o> One<'o> {
        #[allow(clippy::extra_unused_lifetimes)]
        pub fn transitivity<'a, 'b: 'a, 'c: 'b, 'd: 'c, 'e: 'd, 'x>(
            hold: &'x One<'e>,
            nohold: &One<'x>,
        ) -> Box<One<'a>> {
            unsafe { One_transitivity(hold, nohold) }
        }
        #[allow(clippy::extra_unused_lifetimes)]
        pub fn cycle<'a: 'b, 'b: 'c, 'c: 'a, 'x>(
            hold: &Two<'x, 'b>,
            nohold: &'x One<'x>,
        ) -> Box<One<'a>> {
            unsafe { One_cycle(hold, nohold) }
        }
        pub fn many_dependents<'a, 'b: 'a, 'c: 'a, 'd: 'b + 'x, 'x, 'y>(
            a: &'x One<'a>,
            b: &'b One<'a>,
            c: &Two<'x, 'c>,
            d: &'x Two<'d, 'y>,
            nohold: &'x Two<'x, 'y>,
        ) -> Box<One<'a>> {
            unsafe { One_many_dependents(a, b, c, d, nohold) }
        }
        pub fn return_outlives_param<'short, 'long: 'short>(
            hold: &Two<'long, 'short>,
            nohold: &'short One<'short>,
        ) -> Box<One<'long>> {
            unsafe { One_return_outlives_param(hold, nohold) }
        }
        pub fn diamond_top<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'top>> {
            unsafe { One_diamond_top(top, left, right, bottom) }
        }
        pub fn diamond_left<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'left>> {
            unsafe { One_diamond_left(top, left, right, bottom) }
        }
        pub fn diamond_right<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'right>> {
            unsafe { One_diamond_right(top, left, right, bottom) }
        }
        pub fn diamond_bottom<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'bottom>> {
            unsafe { One_diamond_bottom(top, left, right, bottom) }
        }
        pub fn diamond_and_nested_types<'a, 'b: 'a, 'c: 'b, 'd: 'b + 'c, 'x, 'y>(
            a: &One<'a>,
            b: &'y One<'b>,
            c: &One<'c>,
            d: &One<'d>,
            nohold: &One<'x>,
        ) -> Box<One<'a>> {
            unsafe { One_diamond_and_nested_types(a, b, c, d, nohold) }
        }
        #[allow(clippy::extra_unused_lifetimes)]
        pub fn implicit_bounds<'a, 'b: 'a, 'c: 'b, 'd: 'c, 'x, 'y>(
            explicit_hold: &'d One<'x>,
            implicit_hold: &One<'x>,
            nohold: &One<'y>,
        ) -> Box<One<'a>> {
            unsafe { One_implicit_bounds(explicit_hold, implicit_hold, nohold) }
        }
        #[allow(clippy::needless_lifetimes)]
        pub fn implicit_bounds_deep<'a, 'b, 'c, 'd, 'x>(
            explicit_: &'a One<'b>,
            implicit_1: &'b One<'c>,
            implicit_2: &'c One<'d>,
            nohold: &'x One<'x>,
        ) -> Box<One<'a>> {
            unsafe {
                One_implicit_bounds_deep(explicit_, implicit_1, implicit_2, nohold)
            }
        }
    }
    pub struct OpaqueThin(pub crate::lifetimes::Internal);
    impl OpaqueThin {
        pub fn a(&self) -> i32 {
            unsafe { OpaqueThin_a(self) }
        }
        pub fn b(&self) -> f32 {
            unsafe { OpaqueThin_b(self) }
        }
        pub fn c(&self, w: &mut DiplomatWrite) {
            unsafe { OpaqueThin_c(self, w) }
        }
    }
    pub struct OpaqueThinIter<'a>(pub std::slice::Iter<'a, crate::lifetimes::Internal>);
    impl<'a> OpaqueThinIter<'a> {
        pub fn next(&'a mut self) -> Option<&'a OpaqueThin> {
            unsafe { OpaqueThinIter_next(self) }
        }
    }
    pub struct OpaqueThinVec(std::vec::Vec<crate::lifetimes::Internal>);
    impl OpaqueThinVec {
        pub fn create(a: &[i32], b: &[f32], c: &DiplomatStr) -> Box<Self> {
            unsafe { OpaqueThinVec_create(a, b, c) }
        }
        #[allow(clippy::should_implement_trait)]
        pub fn iter<'a>(&'a self) -> Box<OpaqueThinIter<'a>> {
            unsafe { OpaqueThinVec_iter(self) }
        }
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            unsafe { OpaqueThinVec_len(self) }
        }
        pub fn get<'a>(&'a self, idx: usize) -> Option<&'a OpaqueThin> {
            unsafe { OpaqueThinVec_get(self, idx) }
        }
        pub fn first<'a>(&'a self) -> Option<&'a OpaqueThin> {
            unsafe { OpaqueThinVec_first(self) }
        }
    }
}
#[derive(Copy, Clone)]
pub struct One<'a>(&'a ());
#[derive(Copy, Clone)]
pub struct Two<'a, 'b>(&'a (), &'b ());
pub struct Internal {
    a: i32,
    b: f32,
    c: String,
}
