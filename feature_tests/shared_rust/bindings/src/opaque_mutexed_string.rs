use super::Utf16Wrap;
pub struct OpaqueMutexedString;

impl Drop for OpaqueMutexedString {
    fn drop(&mut self) {
        unsafe { OpaqueMutexedString_destroy(self) }
    }
}

impl OpaqueMutexedString {
    pub fn from_usize(number : usize) -> Box<OpaqueMutexedString> {
        let ret = unsafe { OpaqueMutexedString_from_usize(number) };
        ret
    }

    pub fn change<'anon_0>(&'anon_0 self, number : usize) {
        let ret = unsafe { OpaqueMutexedString_change(self, number) };
        ret
    }

    pub fn borrow<'a>(&'a self) -> &'a OpaqueMutexedString {
        let ret = unsafe { OpaqueMutexedString_borrow(self) };
        ret
    }

    pub fn borrow_other<'a>(other : &'a OpaqueMutexedString) -> &'a OpaqueMutexedString {
        let ret = unsafe { OpaqueMutexedString_borrow_other(other) };
        ret
    }

    pub fn borrow_self_or_other<'a>(&'a self, other : &'a OpaqueMutexedString) -> &'a OpaqueMutexedString {
        let ret = unsafe { OpaqueMutexedString_borrow_self_or_other(self, other) };
        ret
    }

    pub fn get_len_and_add<'anon_0>(&'anon_0 self, other : usize) -> usize {
        let ret = unsafe { OpaqueMutexedString_get_len_and_add(self, other) };
        ret
    }

    pub fn dummy_str<'a>(&'a self) -> &'a [u8] {
        let ret = unsafe { OpaqueMutexedString_dummy_str(self) };
        ret
    }

    pub fn wrapper<'a>(&'a self) -> Box<Utf16Wrap> {
        let ret = unsafe { OpaqueMutexedString_wrapper(self) };
        ret
    }

    pub fn to_unsigned_from_unsigned<'anon_0>(&'anon_0 self, input : u16) -> u16 {
        let ret = unsafe { OpaqueMutexedString_to_unsigned_from_unsigned(self, input) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn OpaqueMutexedString_from_usize(number : usize) -> Box<OpaqueMutexedString>;

    fn OpaqueMutexedString_change<'anon_0>(this: &'anon_0 OpaqueMutexedString, number : usize);

    fn OpaqueMutexedString_borrow<'a>(this: &'a OpaqueMutexedString) -> &'a OpaqueMutexedString;

    fn OpaqueMutexedString_borrow_other<'a>(other : &'a OpaqueMutexedString) -> &'a OpaqueMutexedString;

    fn OpaqueMutexedString_borrow_self_or_other<'a>(this: &'a OpaqueMutexedString, other : &'a OpaqueMutexedString) -> &'a OpaqueMutexedString;

    fn OpaqueMutexedString_get_len_and_add<'anon_0>(this: &'anon_0 OpaqueMutexedString, other : usize) -> usize;

    fn OpaqueMutexedString_dummy_str<'a>(this: &'a OpaqueMutexedString) -> &'a diplomat_runtime::DiplomatStrSlice;

    fn OpaqueMutexedString_wrapper<'a>(this: &'a OpaqueMutexedString) -> Box<Utf16Wrap>;

    fn OpaqueMutexedString_to_unsigned_from_unsigned<'anon_0>(this: &'anon_0 OpaqueMutexedString, input : u16) -> u16;

    fn OpaqueMutexedString_destroy(this : *mut OpaqueMutexedString);
}