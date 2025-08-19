use std::borrow::Cow;

use diplomat_core::hir::Method;

pub(super) struct MethodInfo<'a> {
    name : Cow<'a, str>,
    abi_name : Cow<'a, str>,
}

impl<'a> MethodInfo<'a> {
    pub(super) fn gen_method_info(method : &'a Method) {
        
    }
}