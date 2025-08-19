use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir::Method;

#[derive(Template)]
#[template(path = "static_rust/function.rs.jinja", blocks = ["function_impl", "function_def"], escape = "none")]
pub(super) struct FunctionInfo<'tcx> {
    name : Cow<'tcx, str>,
    abi_name : Cow<'tcx, str>,
}

impl<'tcx> FunctionInfo<'tcx> {
    fn gen_function_info(method : &'tcx Method) -> Self {
        FunctionInfo { name: method.name.as_str().into(), abi_name: method.abi_name.as_str().into() }
    }

    pub(super) fn gen_function_block(functions : impl Iterator<Item = &'tcx Method>) -> Vec<FunctionInfo<'tcx>> {
        let mut funcs = Vec::new();
        for f in functions {
            funcs.push(FunctionInfo::gen_function_info(f));
        }
        funcs
    }
}