use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir::{MaybeOwn, Method, Mutability, ReturnType, SelfType, SuccessType, TypeId};

use crate::shared_rust::FileGenContext;

#[derive(Template)]
#[template(path = "shared_rust/function.rs.jinja", blocks = ["function_impl", "function_def"], escape = "none")]
pub(super) struct FunctionInfo<'tcx> {
    name : Cow<'tcx, str>,
    abi_name : Cow<'tcx, str>,
    self_param : Option<ParamInfo<'tcx>>,
    return_type : Option<ParamInfo<'tcx>>,
    params : Vec<ParamInfo<'tcx>>,
    is_write : bool,
}

struct ParamInfo<'a> {
    var_name : Cow<'a, str>,
    type_name : Cow<'a, str>,
    abi_type_override : Option<Cow<'a, str>>,
}

impl<'a> ParamInfo<'a> {
    fn type_name(&self, is_abi : bool) -> Cow<'a, str> {
        if is_abi && self.abi_type_override.is_some() {
            self.abi_type_override.clone().unwrap()
        } else {
            self.type_name.clone()
        }
    }
}

impl<'tcx> FunctionInfo<'tcx> {
    fn gen_function_info(ctx : &mut FileGenContext<'tcx>, method : &'tcx Method) -> Self {
        let mut params = Vec::new();
        for p in &method.params {
            params.push(ParamInfo { var_name: p.name.as_str().into(), type_name: ctx.gen_type_name(&p.ty), abi_type_override: ctx.gen_abi_type_name(&p.ty) });
        }

        let self_param_own = method.param_self.as_ref().map(|s| { 
            match &s.ty {
                SelfType::Opaque(o) => (MaybeOwn::Borrow(o.owner), s.ty.clone()),
                SelfType::Struct(st) => (st.owner, s.ty.clone()),
                SelfType::Enum(..) => (MaybeOwn::Own, s.ty.clone()),
                _ => unreachable!("Unknown SelfType: {:?}", s.ty)
            }
        });

        // TODO: Param/Return type conversions (DiplomatResult and DiplomatOption, basically).
        // I think with `.into()` would be just fine
        let self_param = self_param_own.map(|(s, ty)| {
            let type_name = match ty {
                SelfType::Enum(e) => { 
                    let type_id : TypeId = e.tcx_id.into();
                    ctx.formatter.fmt_symbol_name(type_id.into()) 
                }
                SelfType::Opaque(op) => {
                    let type_id : TypeId = op.tcx_id.into();
                    ctx.formatter.fmt_symbol_name(type_id.into())
                }
                SelfType::Struct(st) => {
                    let type_id : TypeId = st.tcx_id.into();
                    ctx.formatter.fmt_symbol_name(type_id.into())
                }
                _ => unreachable!("Unknown SelfType {ty:?}")
            };

            let (type_name, abi_type) = if s.is_owned() {
                ("self".into(), format!("this : {type_name}"))
            } else {
                let mutable = if s.mutability().is_mutable() {
                    "mut "
                } else { 
                    "" 
                };

                (format!("&{mutable}self"), format!("this: &{mutable}{type_name}"))
            };

            ParamInfo { var_name: "".into(), type_name: type_name.into(), abi_type_override: Some(abi_type.into()) }
        });

        let return_type = Self::gen_return_type_info(&mut params, ctx, &method.output);
        
        FunctionInfo {
            name: method.name.as_str().into(),
            abi_name: method.abi_name.as_str().into(),
            params,
            self_param,
            return_type,
            is_write: method.output.is_write() 
        }
    }
    
    // TODO: &mut DiplomatWrite should only be true for the ABI version of params, not the return.
    fn gen_ok_type_name(params : &mut Vec<ParamInfo>, ctx : &mut FileGenContext<'tcx>, ok : &'tcx SuccessType) -> Cow<'tcx, str>  {
        match ok {
            SuccessType::Unit => "()".into(),
            SuccessType::OutType(o) => {
                // TODO: Opaques.
                ctx.gen_type_name(o)
            }
            SuccessType::Write => {
                params.push(ParamInfo { var_name: "write".into(), type_name: "&mut diplomat_runtime::DiplomatWrite".into(), abi_type_override: None });
                "()".into()
            }
            _ => panic!("HIR SuccessType {ok:?} unsupported")
        }
    }

    fn gen_ok_abi_name(ctx : &mut FileGenContext<'tcx>, ok : &'tcx SuccessType) -> Option<Cow<'tcx, str>> {
        match ok {
            SuccessType::OutType(o) => ctx.gen_abi_type_name(o),
            _ => None
        }
    }

    fn gen_return_type_info(params : &mut Vec<ParamInfo>, ctx : &mut FileGenContext<'tcx>, ret : &'tcx ReturnType) -> Option<ParamInfo<'tcx>> {
         match ret {
            ReturnType::Fallible(ok, err) => {
                let ok_ty = Self::gen_ok_type_name(params, ctx, ok);
                let err_ty = err.as_ref().map(|e| { ctx.gen_type_name(e) }).unwrap_or("()".into());

                let ok_ty_abi = Self::gen_ok_abi_name(ctx, ok);
                let err_ty_abi = err.as_ref().map(|e| { ctx.gen_abi_type_name(e) }).unwrap_or(None);

                let abi_override = format!("diplomat_runtime::DiplomatResult<{}, {}>", ok_ty_abi.unwrap_or(ok_ty.clone()), err_ty_abi.unwrap_or(err_ty.clone()));
                let info = ParamInfo {
                    var_name: "".into(),
                    type_name: format!("Result<{ok_ty}, {err_ty}>").into(),
                    abi_type_override: Some(abi_override.into())
                };
                Some(info)
            }
            ReturnType::Nullable(ok) => {
                let ok_ty = Self::gen_ok_type_name(params, ctx, ok);

                let ok_ty_abi = Self::gen_ok_abi_name(ctx, ok);
                let abi_override = format!("diplomat_runtime::DiplomatOption<{}>", ok_ty_abi.unwrap_or(ok_ty.clone()));

                Some(ParamInfo {
                    var_name: "".into(),
                    type_name: format!("Option<{ok_ty}>").into(),
                    abi_type_override: Some(abi_override.into())
                })
            }
            ReturnType::Infallible(ok) => {
                let type_name = Self::gen_ok_type_name(params, ctx, ok);
                let abi_name = Self::gen_ok_abi_name(ctx, ok);
                if let SuccessType::OutType(..) = ok {
                    Some(ParamInfo { var_name: "".into(), type_name, abi_type_override: abi_name })
                } else {
                    None
                }
            }
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