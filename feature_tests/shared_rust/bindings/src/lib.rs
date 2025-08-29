use std::mem::ManuallyDrop;
use std::ffi::c_void;

pub mod attr_opaque1_renamed;
pub use attr_opaque1_renamed::AttrOpaque1Renamed;

// pub mod bar;
// pub use bar::Bar;

pub mod big_struct_with_stuff;
pub use big_struct_with_stuff::BigStructWithStuff;

// pub mod borrowed_fields;
// pub use borrowed_fields::BorrowedFields;

pub mod borrowed_fields_returning;
pub use borrowed_fields_returning::BorrowedFieldsReturning;

// pub mod borrowed_fields_with_bounds;
// pub use borrowed_fields_with_bounds::BorrowedFieldsWithBounds;

pub mod callback_holder;
pub use callback_holder::CallbackHolder;

pub mod callback_testing_struct;
pub use callback_testing_struct::CallbackTestingStruct;

pub mod callback_wrapper;
pub use callback_wrapper::CallbackWrapper;

pub mod contiguous_enum;
pub use contiguous_enum::ContiguousEnum;

pub mod cyclic_struct_a;
pub use cyclic_struct_a::CyclicStructA;

pub mod cyclic_struct_b;
pub use cyclic_struct_b::CyclicStructB;

pub mod cyclic_struct_c;
pub use cyclic_struct_c::CyclicStructC;

pub mod default_enum;
pub use default_enum::DefaultEnum;

pub mod error_enum;
pub use error_enum::ErrorEnum;

pub mod error_struct;
pub use error_struct::ErrorStruct;

pub mod float64_vec;
pub use float64_vec::Float64Vec;

pub mod float64_vec_error;
pub use float64_vec_error::Float64VecError;

// pub mod foo;
// pub use foo::Foo;

pub mod imported_struct;
pub use imported_struct::ImportedStruct;

pub mod mutable_callback_holder;
pub use mutable_callback_holder::MutableCallbackHolder;

pub mod my_enum;
pub use my_enum::MyEnum;

pub mod my_opaque_enum;
pub use my_opaque_enum::MyOpaqueEnum;

pub mod my_string;
pub use my_string::MyString;

pub mod my_struct;
pub use my_struct::MyStruct;

pub mod my_struct_containing_an_option;
pub use my_struct_containing_an_option::MyStructContainingAnOption;

pub mod my_zst;
pub use my_zst::MyZst;

// pub mod nested_borrowed_fields;
// pub use nested_borrowed_fields::NestedBorrowedFields;

// pub mod one;
// pub use one::One;

pub mod opaque;
pub use opaque::Opaque;

pub mod opaque_mutexed_string;
pub use opaque_mutexed_string::OpaqueMutexedString;

pub mod opaque_thin;
pub use opaque_thin::OpaqueThin;

// pub mod opaque_thin_iter;
// pub use opaque_thin_iter::OpaqueThinIter;

// pub mod opaque_thin_vec;
// pub use opaque_thin_vec::OpaqueThinVec;

pub mod option_enum;
pub use option_enum::OptionEnum;

pub mod option_input_struct;
pub use option_input_struct::OptionInputStruct;

pub mod option_opaque;
pub use option_opaque::OptionOpaque;

pub mod option_opaque_char;
pub use option_opaque_char::OptionOpaqueChar;

pub mod option_string;
pub use option_string::OptionString;

pub(crate) mod option_struct;
pub(crate) use option_struct::OptionStruct;

pub mod primitive_struct;
pub use primitive_struct::PrimitiveStruct;

pub mod primitive_struct_vec;
pub use primitive_struct_vec::PrimitiveStructVec;

pub mod ref_list;
pub use ref_list::RefList;

pub mod ref_list_parameter;
pub use ref_list_parameter::RefListParameter;

pub mod renamed_attr_enum;
pub use renamed_attr_enum::RenamedAttrEnum;

pub mod renamed_attr_opaque2;
pub use renamed_attr_opaque2::RenamedAttrOpaque2;

pub mod renamed_comparable;
pub use renamed_comparable::RenamedComparable;

pub mod renamed_my_indexer;
pub use renamed_my_indexer::RenamedMyIndexer;

pub mod renamed_my_iterable;
pub use renamed_my_iterable::RenamedMyIterable;

pub mod renamed_my_iterator;
pub use renamed_my_iterator::RenamedMyIterator;

pub mod renamed_nested;
pub use renamed_nested::RenamedNested;

pub mod renamed_nested2;
pub use renamed_nested2::RenamedNested2;

pub mod renamed_opaque_arithmetic;
pub use renamed_opaque_arithmetic::RenamedOpaqueArithmetic;

pub mod renamed_opaque_iterable;
pub use renamed_opaque_iterable::RenamedOpaqueIterable;

pub mod renamed_opaque_iterator;
pub use renamed_opaque_iterator::RenamedOpaqueIterator;

pub mod renamed_struct_with_attrs;
pub use renamed_struct_with_attrs::RenamedStructWithAttrs;

pub mod renamed_test_macro_struct;
pub use renamed_test_macro_struct::RenamedTestMacroStruct;

pub mod renamed_test_opaque;
pub use renamed_test_opaque::RenamedTestOpaque;

pub mod renamed_vector_test;
pub use renamed_vector_test::RenamedVectorTest;

pub mod result_opaque;
pub use result_opaque::ResultOpaque;

pub mod scalar_pair_with_padding;
pub use scalar_pair_with_padding::ScalarPairWithPadding;

pub mod struct_arithmetic;
pub use struct_arithmetic::StructArithmetic;

// pub mod struct_with_slices;
// pub use struct_with_slices::StructWithSlices;

pub mod trait_testing_struct;
pub use trait_testing_struct::TraitTestingStruct;

pub mod trait_wrapper;
pub use trait_wrapper::TraitWrapper;

pub mod two;
pub use two::Two;

pub mod unimported_enum;
pub use unimported_enum::UnimportedEnum;

pub mod unnamespaced;
pub use unnamespaced::Unnamespaced;

pub mod utf16_wrap;
pub use utf16_wrap::Utf16Wrap;

#[repr(C)]
struct DiplomatWrite {
    /// Context pointer for additional data needed by `grow()` and `flush()`. May be `null`.
    ///
    /// The pointer may reference structured data on the foreign side,
    /// such as C++ std::string, used to reallocate buf.
    context: *mut c_void,
    /// The raw string buffer, which will be mutated on the Rust side.
    buf: *mut u8,
    /// The current filled size of the buffer
    len: usize,
    /// The current capacity of the buffer
    cap: usize,
    /// Set to true if `grow` ever fails.
    grow_failed: bool,
    /// Called by Rust to indicate that there is no more data to write.
    ///
    /// May be called multiple times.
    ///
    /// Arguments:
    /// - `self` (`*mut DiplomatWrite`): This `DiplomatWrite`
    flush: extern "C" fn(*mut DiplomatWrite),
    /// Called by Rust to request more capacity in the buffer. The implementation should allocate a new
    /// buffer and copy the contents of the old buffer into the new buffer, updating `self.buf` and `self.cap`
    ///
    /// Arguments:
    /// - `self` (`*mut DiplomatWrite`): This `DiplomatWrite`
    /// - `capacity` (`usize`): The requested capacity.
    ///
    /// Returns: `true` if the allocation succeeded. Should not update any state if it failed.
    grow: extern "C" fn(*mut DiplomatWrite, usize) -> bool,
}

impl DiplomatWrite {
    fn new() -> Self {
        let mut str = String::with_capacity(0);

        extern "C" fn grow(this : *mut DiplomatWrite, new_cap : usize) -> bool {
            unsafe {
                let this = this.as_mut().unwrap();
                let mut str = String::from_raw_parts(this.buf, this.len, this.cap);
                str.reserve(new_cap);
                this.cap = str.capacity();
                this.buf = str.as_mut_ptr();
                core::mem::forget(str);
            }
            true
        }
        
        extern "C" fn flush(_: *mut DiplomatWrite) {}

        let out = DiplomatWrite { context: std::ptr::null_mut(),
            buf: str.as_mut_ptr(),
            len: str.len(), cap: str.capacity(),
            grow_failed: false,
            flush, grow };
        
        core::mem::forget(str);
        out
    }

    fn to_string(self) -> String {
        unsafe {
            if !self.buf.is_null() {
                // String takes ownership of the buffer:
                String::from_raw_parts(self.buf, self.len, self.len)
            } else {
                panic!("Could not read buffer, growth failed.")
            }
        }
    }
}

#[repr(C)]
union DiplomatResultValue<T, E> {
    ok: ManuallyDrop<T>,
    err: ManuallyDrop<E>,
}

/// A [`Result`]-like type that can be passed across the FFI boundary
/// as a value. Used internally to return [`Result`]s and [`Option`]s
/// from functions.
#[repr(C)]
struct DiplomatResult<T, E> {
    value: DiplomatResultValue<T, E>,
    pub is_ok: bool,
}

impl<T, E> DiplomatResult<T, E> {
    pub fn to_result(mut self) -> Result<T, E> {
        if self.is_ok {
            Ok(unsafe { ManuallyDrop::take(&mut self.value.ok) })
        } else {
            Err(unsafe { ManuallyDrop::take(&mut self.value.err) })
        }
    }
}