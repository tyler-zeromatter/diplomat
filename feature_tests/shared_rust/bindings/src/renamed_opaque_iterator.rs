use std::marker::PhantomData;

pub struct RenamedOpaqueIterator<'a> {
    a_marker : PhantomData<&'a ()>,
}

impl<'a> Drop for RenamedOpaqueIterator<'a> {
    fn drop(&mut self) {
        unsafe { namespace_OpaqueIterator_destroy(self) }
    }
}

impl<'a> RenamedOpaqueIterator<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_OpaqueIterator_destroy(this : *mut RenamedOpaqueIterator);
}