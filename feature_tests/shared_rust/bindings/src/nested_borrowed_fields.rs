use super::Bar;
use super::BorrowedFields;
use super::BorrowedFieldsWithBounds;
use super::Foo;
use super::borrowed_fields::BorrowedFieldsAbi;
use super::borrowed_fields_with_bounds::BorrowedFieldsWithBoundsAbi;
pub struct NestedBorrowedFields<'x, 'y, 'z> {
    pub fields: BorrowedFields<'x>,
    pub bounds: BorrowedFieldsWithBounds<'x, 'y, 'y>,
    pub bounds2: BorrowedFieldsWithBounds<'z, 'z, 'z>,
}

#[repr(C)]
pub(crate) struct NestedBorrowedFieldsAbi<'x, 'y, 'z> {
    fields : BorrowedFieldsAbi<'x>,
    bounds : BorrowedFieldsWithBoundsAbi<'x, 'y, 'y>,
    bounds2 : BorrowedFieldsWithBoundsAbi<'z, 'z, 'z>,
}

impl<'x, 'y, 'z> NestedBorrowedFieldsAbi<'x, 'y, 'z> {
    pub(crate) fn from_ffi(self) -> NestedBorrowedFields<'x, 'y, 'z> {
        NestedBorrowedFields {
            
            fields: self.fields.from_ffi(),
            
            bounds: self.bounds.from_ffi(),
            
            bounds2: self.bounds2.from_ffi(),
            
        }
    }

    pub(crate) fn to_ffi(this : NestedBorrowedFields<'x, 'y, 'z>) -> NestedBorrowedFieldsAbi<'x, 'y, 'z> {
        NestedBorrowedFieldsAbi {
            
            fields : this.fields.into(),
            
            bounds : this.bounds.into(),
            
            bounds2 : this.bounds2.into(),
            
        }
    }
}

impl<'x, 'y, 'z> From<NestedBorrowedFields<'x, 'y, 'z>> for NestedBorrowedFieldsAbi<'x, 'y, 'z>{
    fn from(value: NestedBorrowedFields<'x, 'y, 'z>) -> Self {
        NestedBorrowedFieldsAbi::to_ffi(value)
    }
}

impl<'x, 'y, 'z> From<NestedBorrowedFieldsAbi<'x, 'y, 'z>> for NestedBorrowedFields<'x, 'y, 'z>{
    fn from(value: NestedBorrowedFieldsAbi<'x, 'y, 'z>) -> Self {
        NestedBorrowedFieldsAbi::from_ffi(value)
    }
}

impl<'x, 'y, 'z> NestedBorrowedFields<'x, 'y, 'z> {
    pub fn from_bar_and_foo_and_strings(bar : &'x Bar<'x, 'y>, foo : &'z Foo<'z>, dstr16_x : &'x [u16], dstr16_z : &'z [u16], utf8_str_y : &'y str, utf8_str_z : &'z str) -> NestedBorrowedFields<'x, 'y, 'z> {
        let ret = unsafe { NestedBorrowedFields_from_bar_and_foo_and_strings(bar, foo, dstr16_x.into(), dstr16_z.into(), utf8_str_y.as_bytes().into(), utf8_str_z.as_bytes().into()) };
        
        ret.from_ffi()
    
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn NestedBorrowedFields_from_bar_and_foo_and_strings<'x, 'y: 'x, 'z>(bar : &'x Bar<'x, 'y>, foo : &'z Foo<'z>, dstr16_x : diplomat_runtime::DiplomatStr16Slice<'x>, dstr16_z : diplomat_runtime::DiplomatStr16Slice<'z>, utf8_str_y : diplomat_runtime::DiplomatStrSlice<'y>, utf8_str_z : diplomat_runtime::DiplomatStrSlice<'z>) -> NestedBorrowedFieldsAbi<'x, 'y, 'z>;
}