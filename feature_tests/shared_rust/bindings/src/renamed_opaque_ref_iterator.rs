use std::marker::PhantomData;

pub struct RenamedOpaqueRefIterator<'a> {
    a_marker : PhantomData<&'a ()>,
}

impl<'a> Drop for RenamedOpaqueRefIterator<'a> {
    fn drop(&mut self) {
        unsafe { namespace_OpaqueRefIterator_destroy(self) }
    }
}

impl<'a> RenamedOpaqueRefIterator<'a> {}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_OpaqueRefIterator_destroy(this : *mut RenamedOpaqueRefIterator);
}