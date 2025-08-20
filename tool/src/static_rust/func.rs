use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir::{MaybeOwn, Method, Mutability, ReturnType, SelfType, SuccessType};

use crate::static_rust::FileGenContext;

#[derive(Template)]
#[template(path = "static_rust/function.rs.jinja", blocks = ["function_impl", "function_def"], escape = "none")]
pub(super) struct FunctionInfo<'tcx> {
    name : Cow<'tcx, str>,
    abi_name : Cow<'tcx, str>,
    self_param : Option<MaybeOwn>,
    return_type : Option<Cow<'tcx, str>>,
    params : Vec<ParamInfo<'tcx>>,
}

struct ParamInfo<'a> {
    var_name : Cow<'a, str>,
    type_name : Cow<'a, str>
}

impl<'tcx> FunctionInfo<'tcx> {
    fn gen_function_info(ctx : &mut FileGenContext<'tcx>, method : &'tcx Method) -> Self {
        let mut params = Vec::new();
        // TODO: out type.
        for p in &method.params {
            params.push(ParamInfo { var_name: p.name.as_str().into(), type_name: ctx.gen_type_name(&p.ty) });
        }

        let self_param = method.param_self.as_ref().map(|s| { 
            match &s.ty {
                SelfType::Opaque(o) => MaybeOwn::Borrow(o.owner),
                SelfType::Struct(st) => st.owner,
                SelfType::Enum(e) => MaybeOwn::Own,
                _ => unreachable!("Unknown SelfType: {:?}", s.ty)
            }
        });

        // TODO: DiplomatOption and DiplomatResult conversion support.
        let return_type = match &method.output {
            ReturnType::Fallible(ok, err) => {
                let ok_ty = Self::gen_ok_type_name(&mut params, ctx, ok);
                let err_ty = err.as_ref().map(|e| { ctx.gen_type_name(e) }).unwrap_or("()".into());
                Some(format!("Result<{ok_ty}, {err_ty}>").into())
            }
            ReturnType::Nullable(ok) => {
                let ok_ty = Self::gen_ok_type_name(&mut params, ctx, ok);
                Some(format!("Option<{ok_ty}>").into())
            }
            ReturnType::Infallible(ok) => {
                let type_name = Self::gen_ok_type_name(&mut params, ctx, ok);
                if let SuccessType::OutType(o) = ok {
                    Some(type_name)
                } else {
                    None
                }
            }
        };

        FunctionInfo { name: method.name.as_str().into(), abi_name: method.abi_name.as_str().into(), params, self_param, return_type }
    }
    
    // TODO: &mut DiplomatWrite should only be true for the ABI version of params, not the return.
    fn gen_ok_type_name(params : &mut Vec<ParamInfo>, ctx : &mut FileGenContext<'tcx>, ok : &'tcx SuccessType) -> Cow<'tcx, str>  {
        match ok {
            SuccessType::Unit => "()".into(),
            SuccessType::OutType(o) => { 
                ctx.gen_type_name(o)
            }
            SuccessType::Write => {
                params.push(ParamInfo { var_name: "output".into(), type_name: "&mut DiplomatWrite".into() });
                "()".into()
            }
            _ => panic!("HIR SuccessType {ok:?} unsupported")
        }
    }

    pub(super) fn gen_function_block(ctx : &mut FileGenContext<'tcx>, functions : impl Iterator<Item = &'tcx Method>) -> Vec<FunctionInfo<'tcx>> {
        let mut funcs = Vec::new();
        for f in functions {
            funcs.push(FunctionInfo::gen_function_info(ctx, f));
        }
        funcs
    }
}