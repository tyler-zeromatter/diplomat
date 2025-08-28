use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir::{
    Lifetime, LifetimeEnv, MaybeOwn, MaybeStatic, Method, OpaqueOwner, ReturnType, SelfType, Slice, SuccessType, TyPosition, Type, TypeId
};

use crate::shared_rust::{formatter::TypeInfo, FileGenContext};

#[derive(Template)]
#[template(path = "shared_rust/function.rs.jinja", blocks = ["function_impl", "function_def"], escape = "none")]
pub(super) struct FunctionInfo<'tcx> {
    name: Cow<'tcx, str>,
    abi_name: Cow<'tcx, str>,
    self_param: Option<ParamInfo<'tcx>>,
    return_type: Option<ParamInfo<'tcx>>,
    params: Vec<ParamInfo<'tcx>>,
    is_write: bool,
    return_info: ReturnType,
    lifetime_env: &'tcx LifetimeEnv,
    generic_lifetimes: Vec<MaybeStatic<Lifetime>>,
}

struct ParamInfo<'a> {
    var_name: Cow<'a, str>,
    type_info: TypeInfo<'a>,
    abi_type_override: Option<Cow<'a, str>>,
    conversion : Option<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> ParamInfo<'a> {
    fn render(&self, env: &LifetimeEnv, is_abi: bool) -> String {
        if is_abi && self.abi_type_override.is_some() {
            self.type_info
                .render_with_override(env, Some(self.abi_type_override.clone().unwrap().into()))
        } else {
            self.type_info.render(env)
        }
    }

    fn wrap_convert(&self) -> Cow<'a, str> {
        let (pre_convert, post_convert) = 
        if let Some((pre, post)) = &self.conversion {
            (pre.clone(), post.clone())
        } else {
            ("".into(), "".into())
        };

        format!("{pre_convert}{}{post_convert}", self.var_name).into()
    }
}

impl<'tcx> FunctionInfo<'tcx> {
    fn render_generic_lifetimes(&self) -> String {
        TypeInfo::fmt_generic_lifetimes(self.generic_lifetimes.clone(), self.lifetime_env)
    }

    fn gen_function_info(ctx: &mut FileGenContext<'tcx>, method: &'tcx Method) -> Self {
        let mut params = Vec::new();
        for p in &method.params {
            params.push(ParamInfo {
                var_name: p.name.as_str().into(),
                type_info: ctx.gen_type_info(&p.ty),
                abi_type_override: ctx.gen_abi_type_name(&p.ty),
                conversion: Self::param_conversion(&p.ty),
            });
        }

        let self_param_own = method.param_self.as_ref().map(|s| match &s.ty {
            SelfType::Opaque(o) => (MaybeOwn::Borrow(o.owner), s.ty.clone()),
            SelfType::Struct(st) => (st.owner, s.ty.clone()),
            SelfType::Enum(..) => (MaybeOwn::Own, s.ty.clone()),
            _ => unreachable!("Unknown SelfType: {:?}", s.ty),
        });

        // TODO: Param/Return type conversions (DiplomatResult and DiplomatOption, basically).
        // I think with `.into()` would be just fine
        let self_param = self_param_own.map(|(s, ty)| {
            let (type_name, self_lifetime) = match ty {
                SelfType::Enum(e) => {
                    let type_id: TypeId = e.tcx_id.into();
                    (ctx.formatter.fmt_symbol_name(type_id.into()), None)
                }
                SelfType::Opaque(op) => {
                    let type_id: TypeId = op.tcx_id.into();
                    (ctx.formatter.fmt_symbol_name(type_id.into()), Some(op.owner.lifetime))
                }
                SelfType::Struct(st) => {
                    let type_id: TypeId = st.tcx_id.into();
                    (ctx.formatter.fmt_symbol_name(type_id.into()), st.owner.lifetime())
                }
                _ => unreachable!("Unknown SelfType {ty:?}"),
            };

            let lt = if let Some(lt) = self_lifetime {
                match lt {
                    MaybeStatic::NonStatic(ns) => { 
                        format!("'{} ", method.lifetime_env.fmt_lifetime(ns))
                    },
                    MaybeStatic::Static => "'static ".into(),
                }
            } else {
                "".into()
            };

            let (type_name, abi_type) = if s.is_owned() {
                ("self".into(), format!("this : {type_name}"))
            } else {
                let mutable = if s.mutability().is_mutable() {
                    "mut "
                } else {
                    ""
                };

                (
                    format!("&{lt}{mutable}self"),
                    format!("this: &{lt}{mutable}{type_name}"),
                )
            };

            ParamInfo {
                var_name: "".into(),
                type_info: TypeInfo::new(type_name.into()),
                abi_type_override: Some(abi_type.into()),
                conversion: None,
            }
        });

        let return_type = Self::gen_return_type_info(&mut params, ctx, &method.output);

        FunctionInfo {
            name: method.name.as_str().into(),
            abi_name: method.abi_name.as_str().into(),
            params,
            self_param,
            return_type,
            is_write: method.output.is_write(),
            return_info: method.output.clone(),
            lifetime_env: &method.lifetime_env,
            // TODO: Need a separate set of lifetimes for the function definition, and one for the ABI.
            generic_lifetimes: method.method_lifetimes().lifetimes().collect(),
        }
    }

    fn param_conversion<P: TyPosition>(ty: &Type<P>) -> Option<(Cow<'tcx, str>, Cow<'tcx, str>)> {
        match ty {
            Type::Slice(sl) => match sl {
                Slice::Str(lt, enc) => {
                    let maybe_enc = if let diplomat_core::hir::StringEncoding::Utf8 = enc {
                        ".as_bytes()"
                    } else {
                        ""
                    };

                    let maybe_borrow = if lt.is_some() {
                        "&"
                    } else {
                        ""
                    };

                    Some((maybe_borrow.into(), format!("{maybe_enc}.into()").into()))
                },
                _ => None,
            },
            _ => None,
        }
    }

    fn gen_ok_type_name(
        params: &mut Vec<ParamInfo>,
        ctx: &mut FileGenContext<'tcx>,
        ok: &'tcx SuccessType,
    ) -> TypeInfo<'tcx> {
        match ok {
            SuccessType::Unit => TypeInfo::new("()".into()),
            SuccessType::OutType(o) => {
                // TODO: Opaques.
                ctx.gen_type_info(o)
            }
            SuccessType::Write => {
                params.push(ParamInfo {
                    var_name: "write_mut".into(),
                    type_info: TypeInfo::new("&mut crate::DiplomatWrite".into()),
                    abi_type_override: None,
                    conversion: None,
                });
                TypeInfo::new("String".into())
            }
            _ => panic!("HIR SuccessType {ok:?} unsupported"),
        }
    }

    fn gen_ok_abi_name(
        ctx: &mut FileGenContext<'tcx>,
        ok: &'tcx SuccessType,
    ) -> Option<Cow<'tcx, str>> {
        match ok {
            SuccessType::OutType(o) => ctx.gen_abi_type_name(o),
            SuccessType::Write => Some("()".into()),
            _ => None,
        }
    }

    fn gen_return_type_info(
        params: &mut Vec<ParamInfo>,
        ctx: &mut FileGenContext<'tcx>,
        ret: &'tcx ReturnType,
    ) -> Option<ParamInfo<'tcx>> {
        match ret {
            ReturnType::Fallible(ok, err) => {
                let ok_ty = Self::gen_ok_type_name(params, ctx, ok);
                let err_ty = err
                    .as_ref()
                    .map(|e| ctx.gen_type_info(e))
                    .unwrap_or(TypeInfo::new("()".into()));

                let ok_ty_abi = Self::gen_ok_abi_name(ctx, ok);
                let err_ty_abi = err
                    .as_ref()
                    .map(|e| ctx.gen_abi_type_name(e))
                    .unwrap_or(None);

                // TODO: Generic lifetimes/borrow information from results (create a `render` function on TypeInfo, recursively use that).
                let abi_override = format!(
                    "crate::DiplomatResult<{}, {}>",
                    ok_ty_abi.unwrap_or(ok_ty.name.clone()),
                    err_ty_abi.unwrap_or(err_ty.name.clone())
                );
                let info = ParamInfo {
                    var_name: "ret".into(),
                    type_info: TypeInfo::new(
                        format!("Result<{}, {}>", ok_ty.name, err_ty.name).into(),
                    ),
                    abi_type_override: Some(abi_override.into()),
                    // TODO: More advanced conversions for inner types.
                    conversion: Some(("".into(), ".into()".into())),
                };
                Some(info)
            }
            ReturnType::Nullable(ok) => {
                let ok_ty = Self::gen_ok_type_name(params, ctx, ok);

                let ok_ty_abi = Self::gen_ok_abi_name(ctx, ok);
                let abi_override = format!(
                    "diplomat_runtime::DiplomatOption<{}>",
                    ok_ty_abi.unwrap_or(ok_ty.name.clone())
                );

                Some(ParamInfo {
                    var_name: "ret".into(),
                    type_info: TypeInfo::new(format!("Option<{}>", ok_ty.name).into()),
                    abi_type_override: Some(abi_override.into()),
                    conversion: Some(("".into(), ".into_converted_option()".into())),
                })
            }
            ReturnType::Infallible(ok) => {
                let type_info = Self::gen_ok_type_name(params, ctx, ok);
                let abi_name = Self::gen_ok_abi_name(ctx, ok);
                if matches!(ok, SuccessType::OutType(..) | SuccessType::Write) {
                    Some(ParamInfo {
                        var_name: "ret".into(),
                        type_info,
                        abi_type_override: abi_name,
                        conversion: None,
                    })
                } else {
                    None
                }
            }
        }
    }

    pub(super) fn gen_function_block(
        ctx: &mut FileGenContext<'tcx>,
        functions: impl Iterator<Item = &'tcx Method>,
    ) -> Vec<FunctionInfo<'tcx>> {
        let mut funcs = Vec::new();
        for f in functions {
            funcs.push(FunctionInfo::gen_function_info(ctx, f));
        }
        funcs
    }
}
