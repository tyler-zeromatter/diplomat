use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir::Method;

use crate::static_rust::FileGenContext;

#[derive(Template)]
#[template(path = "static_rust/function.rs.jinja", blocks = ["function_impl", "function_def"], escape = "none")]
pub(super) struct FunctionInfo<'tcx> {
    name : Cow<'tcx, str>,
    abi_name : Cow<'tcx, str>,
    params : Vec<ParamInfo<'tcx>>,
}

struct ParamInfo<'a> {
    var_name : Cow<'a, str>,
    type_name : String
}

impl<'tcx> FunctionInfo<'tcx> {
    fn gen_function_info(ctx : &mut FileGenContext<'tcx>, method : &'tcx Method) -> Self {
        let mut params = Vec::new();
        // TODO: Self param, out type.
        for p in &method.params {
            params.push(ParamInfo { var_name: p.name.as_str().into(), type_name: ctx.gen_type_name(&p.ty) });
        }
        FunctionInfo { name: method.name.as_str().into(), abi_name: method.abi_name.as_str().into(), params }
    }

    pub(super) fn gen_function_block(ctx : &mut FileGenContext<'tcx>, functions : impl Iterator<Item = &'tcx Method>) -> Vec<FunctionInfo<'tcx>> {
        let mut funcs = Vec::new();
        for f in functions {
            funcs.push(FunctionInfo::gen_function_info(ctx, f));
        }
        funcs
    }
}