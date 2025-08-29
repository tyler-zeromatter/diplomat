use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir::{
    Lifetime, LifetimeEnv, MaybeOwn, MaybeStatic, Method, OpaqueOwner, ReturnType, SelfType, Slice, SpecialMethod, StringEncoding, StructPathLike, SuccessType, SymbolId, TyPosition, Type, TypeDef, TypeId
};

use crate::shared_rust::{
    formatter::{TypeInfo, TypeInfoWrapper},
    FileGenContext, RustFormatter,
};

/// Information need to generate a single function. Used for both the ABI version of the function (`extern "C"`) and the Rust version of the function (`pub fn`).
#[derive(Template, Clone)]
#[template(path = "shared_rust/function.rs.jinja", blocks = ["function_impl", "function_def"], escape = "none")]
pub(super) struct FunctionInfo<'tcx> {
    name: Cow<'tcx, str>,
    abi_name: Cow<'tcx, str>,
    self_param: Option<ParamInfo<'tcx>>,
    params: Vec<ParamInfo<'tcx>>,
    is_write: bool,
    /// Used for displaying type info of the returned type.
    return_type: Option<ParamInfo<'tcx>>,
    /// Used for converting the returned type. Right now, this is only used for `SuccessType::Write` methods, since that gets tricky on the ABI side.
    return_info: ReturnType,
    lifetime_env: &'tcx LifetimeEnv,
    generic_lifetimes: Vec<MaybeStatic<Lifetime>>,
    abi_lifetimes: Vec<MaybeStatic<Lifetime>>,
    special_method : Option<SpecialMethod>,
}

#[derive(Template)]
#[template(path = "shared_rust/special_methods.rs.jinja", escape = "none")]
pub(super) struct SpecialMethodInfo<'tcx> {
    inner : FunctionInfo<'tcx>,
    type_name : Cow<'tcx, str>,
}

#[derive(Default, Clone)]
/// Like [`TypeInfo`], but is used specifically for the `extern "C"` version of a type.
/// Every field will be `None` if there is nothing to override.
pub(super) struct ABITypeInfo<'a> {
    pub(super) name: Option<Cow<'a, str>>,
    pub(super) borrow: Option<MaybeOwn>,
    pub(super) generic_lifetimes: Option<Vec<MaybeStatic<Lifetime>>>,
    pub(super) wrapped: Option<TypeInfoWrapper>,
}

#[derive(Clone)]
struct ParamInfo<'a> {
    var_name: Cow<'a, str>,
    type_info: TypeInfo<'a>,
    abi_override: ABITypeInfo<'a>,
    /// (pre, post) conversion of a type. Can either be to or from the Rust C ABI.
    conversion: Option<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> ParamInfo<'a> {
    // These are both helper functions used in the .jinja templates:
    fn render(&self, env: &LifetimeEnv, is_abi: bool) -> String {
        if is_abi {
            self.type_info.render_with_override(env, &self.abi_override)
        } else {
            self.type_info.render(env)
        }
    }

    fn render_without_borrow(&self, env : &LifetimeEnv, is_abi: bool) -> String {
        if is_abi {
            self.type_info.render_without_borrow(env, &self.abi_override)
        } else {
            self.type_info.render_without_borrow(env, &ABITypeInfo::default())
        }
    }

    fn wrap_convert(&self) -> Cow<'a, str> {
        let (pre_convert, post_convert) = if let Some((pre, post)) = &self.conversion {
            (pre.clone(), post.clone())
        } else {
            ("".into(), "".into())
        };

        format!("{pre_convert}{}{post_convert}", self.var_name).into()
    }
}

impl<'tcx> FunctionInfo<'tcx> {
    /// Jinja Helper
    fn render_generic_lifetimes(&self) -> String {
        TypeInfo::fmt_generic_bounded_lifetimes(self.generic_lifetimes.clone(), self.lifetime_env)
    }
    fn render_abi_generic_lifetimes(&self) -> String {
        TypeInfo::fmt_generic_bounded_lifetimes(self.abi_lifetimes.clone(), self.lifetime_env)
    }

    /// Main constructor for this type. Needs [`FileGenContext`] mostly to be able to update the header.
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
            let (abi_type_name, conversion, self_lifetime) = match ty {
                SelfType::Enum(e) => {
                    let type_id: TypeId = e.tcx_id.into();
                    (ctx.formatter.fmt_symbol_name(type_id.into()), None, None)
                }
                SelfType::Opaque(op) => {
                    let type_id: TypeId = op.tcx_id.into();
                    (
                        ctx.formatter.fmt_symbol_name(type_id.into()),
                        None,
                        Some(op.owner.lifetime),
                    )
                }
                SelfType::Struct(st) => {
                    let type_id: TypeId = st.tcx_id.into();
                    let name = ctx.formatter.fmt_symbol_name(type_id.into());
                    (
                        ctx.formatter.fmt_struct_abi_name(name),
                        Some(("".into(), ".into()".into())),
                        st.owner.lifetime(),
                    )
                }
                _ => unreachable!("Unknown SelfType {ty:?}"),
            };

            let lt = if let Some(lt) = self_lifetime {
                match lt {
                    MaybeStatic::NonStatic(ns) => {
                        format!("'{} ", method.lifetime_env.fmt_lifetime(ns))
                    }
                    MaybeStatic::Static => "'static ".into(),
                }
            } else {
                "".into()
            };

            let (type_name, abi_type_name) = if s.is_owned() {
                ("self".into(), format!("this : {abi_type_name}"))
            } else {
                let mutable = if s.mutability().is_mutable() {
                    "mut "
                } else {
                    ""
                };

                (
                    format!("&{lt}{mutable}self"),
                    format!("this: &{lt}{mutable}{abi_type_name}"),
                )
            };

            let abi_info = ABITypeInfo {
                name: Some(abi_type_name.into()),
                ..Default::default()
            };

            ParamInfo {
                var_name: "self".into(),
                type_info: TypeInfo::new(type_name.into()),
                abi_override: abi_info,
                conversion,
            }
        });

        let return_type =
            Self::gen_return_type_info(&mut params, ctx, &method.output, &method.lifetime_env);

        // FIXME: This is hacky and ugly, I think a real design discussion about how to avoid lifetimes already existing in the SelfType's impl block is needed.
        let formatted_parent = if let SymbolId::TypeId(ty) = ctx.id {
            match ctx.tcx.resolve_type(ty) {
                TypeDef::Opaque(op) => op
                    .lifetimes
                    .all_lifetimes()
                    .map(|l| op.lifetimes.fmt_lifetime(l))
                    .collect(),
                TypeDef::Struct(st) => st
                    .lifetimes
                    .all_lifetimes()
                    .map(|l| st.lifetimes.fmt_lifetime(l))
                    .collect(),
                TypeDef::OutStruct(st) => st
                    .lifetimes
                    .all_lifetimes()
                    .map(|l| st.lifetimes.fmt_lifetime(l))
                    .collect(),
                _ => Vec::new(),
            }
        } else {
            Vec::new()
        };
        let method_lifetimes = method.method_lifetimes();

        let lifetimes = method_lifetimes.lifetimes().filter(|lt| match lt {
            MaybeStatic::Static => true,
            MaybeStatic::NonStatic(ns) => {
                !formatted_parent.contains(&method.lifetime_env.fmt_lifetime(ns))
            }
        });

        FunctionInfo {
            name: method.name.as_str().into(),
            abi_name: method.abi_name.as_str().into(),
            params,
            self_param,
            return_type,
            is_write: method.output.is_write(),
            return_info: method.output.clone(),
            lifetime_env: &method.lifetime_env,
            // TODO: Bounded lifetimes.
            generic_lifetimes: lifetimes.collect(),
            abi_lifetimes: method_lifetimes.lifetimes().collect(),
            special_method: method.attrs.special_method.clone(),
        }
    }

    /// Get the (pre, post) conversion from Rust to the C ABI.
    pub(super) fn param_conversion<P: TyPosition>(
        ty: &Type<P>,
    ) -> Option<(Cow<'tcx, str>, Cow<'tcx, str>)> {
        match ty {
            Type::Slice(sl) => match sl {
                Slice::Str(..) => {
                    //  &str|&[u8] -> DiplomatStrSlice
                    Some(("".into(), format!(".into()").into()))
                }
                // From &[String|&[u8]|&[u16]] -> DiplomatSlice<DiplomatStrSlice>
                Slice::Strs(enc) => {
                    let name = RustFormatter::fmt_slice_abi_name(enc);

                    // FIXME: Incredibly hacky.
                    let convert = format!(".iter().map(|u| {{ {name}::from(*u) }}).collect::<Vec<_>>().as_slice().into()");

                    Some(("".into(), convert.into()))
                }
                _ => None,
            },
            Type::DiplomatOption(ok) => {
                // Option<T> -> DiplomatOption<U>
                let maybe_map = if let Some((pre, post)) = Self::param_conversion(ok) {
                    format!(".map(|ok| {{ {pre}ok{post} }})")
                } else {
                    "".into()
                };
                Some(("".into(), format!("{maybe_map}.into()").into()))
            }
            Type::Struct(..) => {
                // Struct -> StructAbi
                Some(("".into(), ".into()".into()))
            }
            _ => None,
        }
    }

    /// For a given [`SuccessType`], get the Rust type information.
    fn gen_ok_type_info(
        params: &mut Vec<ParamInfo>,
        ctx: &mut FileGenContext<'tcx>,
        ok: &'tcx SuccessType,
    ) -> TypeInfo<'tcx> {
        match ok {
            SuccessType::Unit => TypeInfo::new("()".into()),
            SuccessType::OutType(o) => ctx.gen_type_info(o),
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

    /// For a given [`SuccessType`], get the C ABI type information.
    fn gen_ok_abi_info(ctx: &mut FileGenContext<'tcx>, ok: &'tcx SuccessType) -> ABITypeInfo<'tcx> {
        match ok {
            SuccessType::OutType(o) => Self::gen_abi_type_info(ctx, o),
            SuccessType::Write => ABITypeInfo {
                name: Some("()".into()),
                ..Default::default()
            },
            _ => ABITypeInfo::default(),
        }
    }

    /// Get the C ABI -> Rust conversion for either a return type or a struct's fields.
    pub(super) fn out_type_conversion<P: TyPosition>(
        out: &Type<P>,
    ) -> Option<(Cow<'tcx, str>, Cow<'tcx, str>)> {
        match out {
            Type::Slice(Slice::Str(lt, enc)) if lt.is_some() => match enc {
                // From DiplomatUtf8SliceStr -> &str
                StringEncoding::Utf8 => Some((
                    "".into(),
                    ".into()".into(),
                )),
                // For any other kind of string conversion, we want to convert from `DiplomatSliceStr` -> &[u8] or &[u16]:
                _ => Some(("".into(), ".into()".into())),
            },
            // StructAbi -> Struct
            Type::Struct(..) => Some(("".into(), ".from_ffi()".into())),
            // DiplomatOption<T> -> Option<U>
            Type::DiplomatOption(..) => Some(("".into(), ".into_converted_option()".into())),
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
        env: &LifetimeEnv,
    ) -> Option<ParamInfo<'tcx>> {
        match ret {
            ReturnType::Fallible(ok, err) => {
                let ok_ty = Self::gen_ok_type_info(params, ctx, ok);
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
                    name: Some(
                        format!(
                            "crate::DiplomatResult<{}, {}>",
                            ok_ty.render_with_override(env, &ok_ty_abi),
                            err_ty.render_with_override(env, &err_ty_abi)
                        )
                        .into(),
                    ),
                    ..Default::default()
                };

                let ok_convert = Self::ok_type_conversion(ok);
                let err_convert = err.as_ref().and_then(|e| Self::out_type_conversion(e));
                let maybe_map_ok = if let Some((pre, post)) = ok_convert {
                    format!(
                        ".map(|ok : {}| {{ {pre}ok{post} }})",
                        ok_ty_abi.name.unwrap()
                    )
                } else {
                    "".into()
                };
                let maybe_map_err = if let Some((pre, post)) = err_convert {
                    format!(
                        ".map_err(|err : {}| {{ {pre}err{post} }})",
                        err_ty_abi.name.unwrap()
                    )
                } else {
                    "".into()
                };

                let info = ParamInfo {
                    var_name: "ret".into(),
                    type_info: TypeInfo::new(
                        format!("Result<{}, {}>", ok_ty.render(env), err_ty.render(env)).into(),
                    ),
                    abi_override: abi_override,
                    conversion: Some((
                        "".into(),
                        format!(".to_result(){maybe_map_ok}{maybe_map_err}").into(),
                    )),
                };
                Some(info)
            }
            ReturnType::Nullable(ok) => {
                let ok_ty = Self::gen_ok_type_info(params, ctx, ok);

                let ok_ty_abi = Self::gen_ok_abi_info(ctx, ok);
                let abi_override = ABITypeInfo {
                    name: Some(
                        format!(
                            "diplomat_runtime::DiplomatOption<{}>",
                            ok_ty.render_with_override(env, &ok_ty_abi)
                        )
                        .into(),
                    ),
                    ..Default::default()
                };

                let convert = Self::ok_type_conversion(ok);
                let potential_map = if let Some((pre, post)) = convert {
                    format!(
                        ".map(|ok : {}| {{ {pre}ok{post} }})",
                        ok_ty_abi.name.unwrap()
                    )
                } else {
                    "".into()
                };

                Some(ParamInfo {
                    var_name: "ret".into(),
                    type_info: TypeInfo::new(format!("Option<{}>", ok_ty.render(env)).into()),
                    abi_override,
                    conversion: Some((
                        "".into(),
                        format!(".into_converted_option(){potential_map}").into(),
                    )),
                })
            }
            ReturnType::Infallible(ok) => {
                let type_info = Self::gen_ok_type_info(params, ctx, ok);
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

    /// Generate an impl block for special Rust trait stuff.
    /// Assumes that any special method can be generated as an `impl` trait block separately from the original method definition, and just call into that.
    /// 
    /// TODO: If you're interested in hiding the underlying conversion function, I'd add a `vis` modifier to [`FunctionInfo`] and make `functions` a mutable reference.
    pub(super) fn get_special_methods(
        ctx : &mut FileGenContext,
        functions: Vec<FunctionInfo<'tcx>>,
        self_type : Cow<'tcx, str>,
    ) -> Vec<SpecialMethodInfo<'tcx>> {
        let mut special_methods = Vec::new();
        for f in functions {
            if let Some(special_method) = &f.special_method {
                // TODO: `impl Index` is not, unfortunately, super easy to implement.
                // special_methods.push(SpecialMethodInfo { inner: f, type_name: self_type.clone() })
            }
        }
        special_methods
    }

    /// Given any type, generate C ABI info.
    /// TODO: Should this be moved to [`FileGenContext`], since it's used both by [`ParamInfo`] and `FieldInfo`?
    pub(super) fn gen_abi_type_info<P: TyPosition>(
        ctx: &mut FileGenContext,
        ty: &Type<P>,
    ) -> ABITypeInfo<'tcx> {
        match ty {
            Type::DiplomatOption(op) => {
                let regular_type = ctx.gen_type_info(op).name;
                let inner = Self::gen_abi_type_info(ctx, op);
                ABITypeInfo {
                    name: Some(
                        format!(
                            "diplomat_runtime::DiplomatOption<{}>",
                            inner.name.unwrap_or(regular_type)
                        )
                        .into(),
                    ),
                    ..Default::default()
                }
            }
            Type::Slice(sl) => match sl {
                Slice::Str(lt, enc) => {
                    let name = if lt.is_none() {
                        RustFormatter::fmt_owned_slice_abi_name(enc)
                    } else {
                        RustFormatter::fmt_slice_abi_name(enc)
                    };

                    ABITypeInfo {
                        name: Some(name.into()),
                        // We move the borrow to the generic lifetimes:
                        borrow: Some(MaybeOwn::Own),
                        generic_lifetimes: Some(lt.iter().cloned().collect()),
                        // We want to avoid any Box<> issues for owned slices (since we already own this with DiplomatSlice):
                        wrapped: Some(TypeInfoWrapper::None),
                    }
                }
                Slice::Strs(enc) => {
                    let name = RustFormatter::fmt_slice_abi_name(enc);

                    let name = format!("diplomat_runtime::DiplomatSlice<{name}>");
                    ABITypeInfo {
                        name: Some(name.into()),
                        ..Default::default()
                    }
                }
                _ => ABITypeInfo::default(),
            },
            Type::Struct(st) => {
                let struct_name = ctx.formatter.fmt_symbol_name(st.id().into());
                let name = ctx
                    .formatter
                    .fmt_struct_abi_name(struct_name.clone())
                    .into_owned();
                // FIXME: Hacky, needs to be able to take into account namespaces
                ctx.add_import(format!("{}::{name}", heck::AsSnakeCase(struct_name)));
                ABITypeInfo {
                    name: Some(name.into()),
                    ..Default::default()
                }
            }
            _ => ABITypeInfo::default(),
        }
    }
}
