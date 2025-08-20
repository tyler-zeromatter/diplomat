use super::Utf16Wrap;
pub struct OpaqueMutexedString;

impl OpaqueMutexedString {
    fn from_usize(number : usize) -> OpaqueMutexedString {
            // TODO: writeable conversions.
        unsafe { OpaqueMutexedString_from_usize(number) }
    }

    fn change(&self, number : usize) {
            // TODO: writeable conversions.
        unsafe { OpaqueMutexedString_change(self, number) }
    }

    fn borrow(&self) -> OpaqueMutexedString {
            // TODO: writeable conversions.
        unsafe { OpaqueMutexedString_borrow(self) }
    }

    fn borrow_other(other : OpaqueMutexedString) -> OpaqueMutexedString {
            // TODO: writeable conversions.
        unsafe { OpaqueMutexedString_borrow_other(other) }
    }

    fn borrow_self_or_other(&self, other : OpaqueMutexedString) -> OpaqueMutexedString {
            // TODO: writeable conversions.
        unsafe { OpaqueMutexedString_borrow_self_or_other(self, other) }
    }

    fn get_len_and_add(&self, other : usize) -> usize {
            // TODO: writeable conversions.
        unsafe { OpaqueMutexedString_get_len_and_add(self, other) }
    }

    fn dummy_str(&self) -> &[TODO] {
            // TODO: writeable conversions.
        unsafe { OpaqueMutexedString_dummy_str(self) }
    }

    fn wrapper(&self) -> Utf16Wrap {
            // TODO: writeable conversions.
        unsafe { OpaqueMutexedString_wrapper(self) }
    }

    fn to_unsigned_from_unsigned(&self, input : u16) -> u16 {
            // TODO: writeable conversions.
        unsafe { OpaqueMutexedString_to_unsigned_from_unsigned(self, input) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OpaqueMutexedString_from_usize(number : usize) -> OpaqueMutexedString;

    fn OpaqueMutexedString_change(this: &OpaqueMutexedString, number : usize);

    fn OpaqueMutexedString_borrow(this: &OpaqueMutexedString) -> OpaqueMutexedString;

    fn OpaqueMutexedString_borrow_other(other : OpaqueMutexedString) -> OpaqueMutexedString;

    fn OpaqueMutexedString_borrow_self_or_other(this: &OpaqueMutexedString, other : OpaqueMutexedString) -> OpaqueMutexedString;

    fn OpaqueMutexedString_get_len_and_add(this: &OpaqueMutexedString, other : usize) -> usize;

    fn OpaqueMutexedString_dummy_str(this: &OpaqueMutexedString) -> &[TODO];

    fn OpaqueMutexedString_wrapper(this: &OpaqueMutexedString) -> Utf16Wrap;

    fn OpaqueMutexedString_to_unsigned_from_unsigned(this: &OpaqueMutexedString, input : u16) -> u16;

}