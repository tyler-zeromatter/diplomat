use std::marker::PhantomData;

pub struct RenamedMyIterator<'a> {
    a_marker : PhantomData<&'a ()>,
}

impl<'a> Drop for RenamedMyIterator<'a> {
    fn drop(&mut self) {
        unsafe { namespace_MyIterator_destroy(self) }
    }
}

impl<'a> RenamedMyIterator<'a> {
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_MyIterator_destroy(this : *mut RenamedMyIterator);
}