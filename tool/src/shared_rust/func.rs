use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir::{
    Lifetime, LifetimeEnv, MaybeOwn, MaybeStatic, Method, OpaqueOwner, ReturnType, SelfType, Slice, StringEncoding, SuccessType, TyPosition, Type, TypeId
};

use crate::shared_rust::{formatter::{TypeInfo, TypeInfoWrapper}, FileGenContext};

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

#[derive(Default)]
pub(super) struct ABITypeInfo<'a> {
    pub(super) name : Option<Cow<'a, str>>,
    pub(super) borrow : Option<MaybeOwn>,
    pub(super) generic_lifetimes : Option<Vec<MaybeStatic<Lifetime>>>,
    pub(super) wrapped : Option<TypeInfoWrapper>,
}

struct ParamInfo<'a> {
    var_name: Cow<'a, str>,
    type_info: TypeInfo<'a>,
    abi_override: ABITypeInfo<'a>,
    conversion : Option<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> ParamInfo<'a> {
    fn render(&self, env: &LifetimeEnv, is_abi: bool) -> String {
        if is_abi {
            self.type_info
                .render_with_override(env, &self.abi_override)
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
        TypeInfo::fmt_generic_bounded_lifetimes(self.generic_lifetimes.clone(), self.lifetime_env)
    }

    fn gen_function_info(ctx: &mut FileGenContext<'tcx>, method: &'tcx Method) -> Self {
        let mut params = Vec::new();
        for p in &method.params {
            params.push(ParamInfo {
                var_name: p.name.as_str().into(),
                type_info: ctx.gen_type_info(&p.ty),
                abi_override: Self::gen_abi_type_info(ctx, &p.ty),
                conversion: Self::param_conversion(&p.ty),
            });
        }

        let self_param_own = method.param_self.as_ref().map(|s| match &s.ty {
            SelfType::Opaque(o) => (MaybeOwn::Borrow(o.owner), s.ty.clone()),
            SelfType::Struct(st) => (st.owner, s.ty.clone()),
            SelfType::Enum(..) => (MaybeOwn::Own, s.ty.clone()),
            _ => unreachable!("Unknown SelfType: {:?}", s.ty),
        });

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

            let (type_name, abi_type_name) = if s.is_owned() {
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

            let abi_info = ABITypeInfo {
                name: Some(abi_type_name.into()),
                ..Default::default()
            };

            ParamInfo {
                var_name: "".into(),
                type_info: TypeInfo::new(type_name.into()),
                abi_override: abi_info,
                conversion: None,
            }
        });

        let return_type = Self::gen_return_type_info(&mut params, ctx, &method.output, &method.lifetime_env);

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
            // TODO: Bounded lifetimes.
            generic_lifetimes: method.method_lifetimes().lifetimes().collect(),
        }
    }

    fn param_conversion<P: TyPosition>(ty: &Type<P>) -> Option<(Cow<'tcx, str>, Cow<'tcx, str>)> {
        match ty {
            Type::Slice(sl) => match sl {
                Slice::Str(_, enc) => {
                    let maybe_enc = if let diplomat_core::hir::StringEncoding::Utf8 = enc {
                        // From String or &str -> &[u8]:
                        ".as_bytes()"
                    } else {
                        ""
                    };

                    Some(("".into(), format!("{maybe_enc}.into()").into()))
                },
                _ => None,
            },
            Type::DiplomatOption(..) => Some(("".into(), ".into()".into())),
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
                ctx.gen_type_info(o)
            }
            SuccessType::Write => {
                params.push(ParamInfo {
                    var_name: "write_mut".into(),
                    type_info: TypeInfo::new("&mut crate::DiplomatWrite".into()),
                    abi_override: ABITypeInfo::default(),
                    conversion: None,
                });
                TypeInfo::new("String".into())
            }
            _ => panic!("HIR SuccessType {ok:?} unsupported"),
        }
    }

    fn gen_ok_abi_info(
        ctx: &mut FileGenContext<'tcx>,
        ok: &'tcx SuccessType,
    ) -> ABITypeInfo<'tcx> {
        match ok {
            SuccessType::OutType(o) => Self::gen_abi_type_info(ctx, o),
            SuccessType::Write => ABITypeInfo { name: Some("()".into()), ..Default::default() },
            _ => ABITypeInfo::default(),
        }
    }

    fn out_type_conversion<P: TyPosition>(out : &Type<P>) -> Option<(Cow<'tcx, str>, Cow<'tcx, str>)> {
        match out {
            Type::Slice(Slice::Str(lt, enc)) if lt.is_some() => match enc {
                // ABI returns DiplomatSliceStr, we want -> &[u8] -> &str
                StringEncoding::Utf8 => Some(("unsafe { str::from_utf8_unchecked(".into(), ".into()).into()}".into())),
                // For any other kind of string conversion, we want to convert from `DiplomatSliceStr` -> &[u8] or &[u16]:
                _ => Some(("".into(), ".into()".into())),
            },
            _ => None,
        }
    }

    fn ok_type_conversion(ok: &'tcx SuccessType) -> Option<(Cow<'tcx, str>, Cow<'tcx, str>)> {
        match ok {
            SuccessType::OutType(o) => Self::out_type_conversion(o),
            _ => None,
        }
    }

    fn gen_return_type_info(
        params: &mut Vec<ParamInfo>,
        ctx: &mut FileGenContext<'tcx>,
        ret: &'tcx ReturnType,
        env : &LifetimeEnv,
    ) -> Option<ParamInfo<'tcx>> {
        match ret {
            ReturnType::Fallible(ok, err) => {
                let ok_ty = Self::gen_ok_type_name(params, ctx, ok);
                let err_ty = err
                    .as_ref()
                    .map(|e| ctx.gen_type_info(e))
                    .unwrap_or(TypeInfo::new("()".into()));

                let ok_ty_abi = Self::gen_ok_abi_info(ctx, ok);
                let err_ty_abi = err
                    .as_ref()
                    .map(|e| Self::gen_abi_type_info(ctx, e))
                    .unwrap_or_default();

                let abi_override = ABITypeInfo {
                    name: Some(format!(
                        "crate::DiplomatResult<{}, {}>",
                        ok_ty.render_with_override(env, &ok_ty_abi),
                        err_ty.render_with_override(env, &err_ty_abi)
                    ).into()),
                    ..Default::default()
                };
                
                let ok_convert = Self::ok_type_conversion(ok);
                let err_convert = err.as_ref().and_then(|e| {
                    Self::out_type_conversion(e)
                });
                let maybe_map_ok = if let Some((pre, post)) = ok_convert {
                    format!(".map(|ok : {}| {{ {pre}ok{post} }})", ok_ty_abi.name.unwrap())
                } else {
                    "".into()
                };
                let maybe_map_err = if let Some((pre, post)) = err_convert {
                    format!(".map_err(|err : {}| {{ {pre}err{post} }})", err_ty_abi.name.unwrap())
                } else {
                    "".into()
                };

                let info = ParamInfo {
                    var_name: "ret".into(),
                    type_info: TypeInfo::new(
                        format!("Result<{}, {}>", ok_ty.render(env), err_ty.render(env)).into(),
                    ),
                    abi_override: abi_override,
                    conversion: Some(("".into(), format!(".to_result(){maybe_map_ok}{maybe_map_err}").into())),
                };
                Some(info)
            }
            ReturnType::Nullable(ok) => {
                let ok_ty = Self::gen_ok_type_name(params, ctx, ok);

                let ok_ty_abi = Self::gen_ok_abi_info(ctx, ok);
                let abi_override = ABITypeInfo {
                    name: Some(format!(
                        "diplomat_runtime::DiplomatOption<{}>",
                        ok_ty.render_with_override(env, &ok_ty_abi)
                    ).into()),
                    ..Default::default()
                };

                let convert = Self::ok_type_conversion(ok);
                let potential_map = if let Some((pre, post)) = convert {
                    format!(".map(|ok : {}| {{ {pre}ok{post} }})", ok_ty_abi.name.unwrap())
                } else {
                    "".into()
                };

                Some(ParamInfo {
                    var_name: "ret".into(),
                    type_info: TypeInfo::new(format!("Option<{}>", ok_ty.render(env)).into()),
                    abi_override,
                    conversion: Some(("".into(), format!(".into_converted_option(){potential_map}").into())),
                })
            }
            ReturnType::Infallible(ok) => {
                let type_info = Self::gen_ok_type_name(params, ctx, ok);
                let abi_name = Self::gen_ok_abi_info(ctx, ok);

                if matches!(ok, SuccessType::OutType(..) | SuccessType::Write) {
                    Some(ParamInfo {
                        var_name: "ret".into(),
                        type_info,
                        abi_override: abi_name,
                        conversion: Self::ok_type_conversion(ok),
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

    fn gen_abi_type_info<P: TyPosition>(
        ctx : &mut FileGenContext,
        ty: &Type<P>,
    ) -> ABITypeInfo<'tcx> {
        match ty {
            Type::DiplomatOption(op) => {
                let regular_type = ctx.gen_type_info(op).name;
                let inner = Self::gen_abi_type_info(ctx, op);
                ABITypeInfo {
                    name: Some(format!(
                        "diplomat_runtime::DiplomatOption<{}>",
                        inner.name.unwrap_or(regular_type)
                    )
                    .into()),
                    ..Default::default()
                }
            }
            Type::Slice(sl) => match sl {
                Slice::Str(lt, enc) => {
                    let name = match enc {
                        StringEncoding::Utf8 | StringEncoding::UnvalidatedUtf8 => "diplomat_runtime::DiplomatStrSlice",
                        StringEncoding::UnvalidatedUtf16 => "diplomat_runtime::DiplomatStr16Slice",
                        _ => panic!("Unrecognized encoding type {enc:?}"),
                    };

                    let name = if lt.is_none() {
                        match enc {
                            StringEncoding::Utf8 | StringEncoding::UnvalidatedUtf8 => "diplomat_runtime::DiplomatOwnedStrSlice",
                            StringEncoding::UnvalidatedUtf16 => "diplomat_runtime::DiplomatOwnedStr16Slice",
                            _ => panic!("Unrecognized encoding type {enc:?}"),
                        }
                    } else {
                        name
                    };

                    ABITypeInfo {
                        name: Some(name.into()),
                        // We move the borrow to the generic lifetimes:
                        borrow: Some(MaybeOwn::Own),
                        generic_lifetimes: Some(lt.iter().cloned().collect()),
                        wrapped: Some(TypeInfoWrapper::None),
                    }
                },
                _ => ABITypeInfo::default(),
            },
            _ => ABITypeInfo::default(),
        }
    }
}
