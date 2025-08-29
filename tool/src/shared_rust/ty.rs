use std::{borrow::Cow, collections::BTreeSet};

use askama::Template;
use diplomat_core::hir::{
    EnumDef, Lifetime, LifetimeEnv, MaybeOwn, MaybeStatic, OpaqueDef, OpaqueOwner, Slice,
    StringEncoding, StructDef, StructPathLike, SymbolId, TyPosition, Type, TypeContext, TypeId,
};

use crate::{
    config::Config,
    shared_rust::{
        formatter::{TypeInfo, TypeInfoWrapper},
        func::{ABITypeInfo, FunctionInfo},
        RustFormatter,
    },
};

/// Generation context of a single `.rs` file to be output.
/// Currently implemented for files containing `struct` or `enum` information,
/// although this should (and is designed to) be expanded to include free functions.
pub(super) struct FileGenContext<'tcx> {
    pub(super) formatter: &'tcx RustFormatter<'tcx>,
    pub(super) tcx: &'tcx TypeContext,
    pub(super) id: SymbolId,
    /// For the #[link(name=)] attribute.
    lib_name: String,
    /// All the `use` statements at the start of the file.
    /// TODO: For future namespacing support, this (and [`TypeInfo`]) will need to be updated,
    /// especially to account for types of the same name but in different namespaces.
    imports: BTreeSet<String>,
}

/// Helper trait for file generation templates.
/// Since not all generated file templates will be the same,
/// this is a quick helper to access any important shared details after generating information.
pub(super) trait TypeTemplate<'a> {
    fn render(&self) -> askama::Result<String>;
    fn imports(&mut self) -> &mut BTreeSet<String>;
    fn mod_name(&self) -> String;
    /// Lifetimes on the impl block
    /// TODO: Add a bounded_generic_lifetime function as well.
    fn generic_lifetimes(&self) -> String;
}

impl<'tcx, 'rcx> FileGenContext<'tcx> {
    /// TODO: Create a version of this for free functions.
    /// Constructor from a given [`TypeId`].
    pub(super) fn from_type<'a>(
        config: &Config,
        id: TypeId,
        formatter: &'a RustFormatter,
        tcx: &'a TypeContext,
    ) -> FileGenContext<'a> {
        FileGenContext {
            formatter,
            id: id.into(),
            tcx,
            lib_name: config
                .shared_config
                .lib_name
                .clone()
                .expect("Rust static backend needs lib_name to link against."),
            imports: BTreeSet::new(),
        }
    }

    pub(super) fn generate_struct<P: TyPosition>(
        mut self,
        ty: &'tcx StructDef<P>,
        is_out: bool,
    ) -> impl TypeTemplate<'tcx> {
        /// Like [`super::func::ParamInfo`], and re-uses a lot of the same methods that `ParamInfo` does for generation,
        /// except it needs to be able to convert to and from C/Rust.
        struct FieldInfo<'a> {
            type_info: TypeInfo<'a>,
            abi_info: ABITypeInfo<'a>,
            name: Cow<'a, str>,
            to_rust: Option<(Cow<'a, str>, Cow<'a, str>)>,
            to_c_abi: Option<(Cow<'a, str>, Cow<'a, str>)>,
        }

        impl<'a> FieldInfo<'a> {
            fn wrap_convert(&self) -> Cow<'a, str> {
                let (pre_convert, post_convert) = if let Some((pre, post)) = &self.to_rust {
                    (pre.clone(), post.clone())
                } else {
                    ("".into(), "".into())
                };

                format!("{pre_convert}self.{}{post_convert}", self.name).into()
            }

            fn wrap_c_convert(&self) -> Cow<'a, str> {
                let (pre_convert, post_convert) = if let Some((pre, post)) = &self.to_c_abi {
                    (pre.clone(), post.clone())
                } else {
                    ("".into(), "".into())
                };

                format!("{pre_convert}this.{}{post_convert}", self.name).into()
            }
        }

        #[derive(Template)]
        #[template(path = "shared_rust/struct.rs.jinja", escape = "none")]
        struct StructTemplate<'a> {
            type_name: Cow<'a, str>,
            methods: Vec<FunctionInfo<'a>>,
            lib_name: String,
            imports: BTreeSet<String>,
            is_out: bool,
            fields: Vec<FieldInfo<'a>>,
            lifetime_env: &'a LifetimeEnv,
            lifetimes: Vec<Lifetime>,
        }

        let methods = FunctionInfo::gen_function_block(&mut self, ty.methods.iter());

        let lifetime_env = &ty.lifetimes;
        let lifetimes = lifetime_env.all_lifetimes().collect();
        // TODO: Bounded lifetimes for the impl block.

        let fields = ty
            .fields
            .iter()
            .map(|f| FieldInfo {
                type_info: self.gen_type_info(&f.ty),
                abi_info: FunctionInfo::gen_abi_type_info(&mut self, &f.ty),
                to_rust: FunctionInfo::out_type_conversion(&f.ty),
                to_c_abi: FunctionInfo::param_conversion(&f.ty),
                name: f.name.as_str().into(),
            })
            .collect();

        impl<'a> TypeTemplate<'a> for StructTemplate<'a> {
            fn render(&self) -> askama::Result<String> {
                askama::Template::render(self)
            }
            fn imports(&mut self) -> &mut BTreeSet<String> {
                &mut self.imports
            }
            fn mod_name(&self) -> String {
                self.type_name.clone().into()
            }

            fn generic_lifetimes(&self) -> String {
                let new_lts = self
                    .lifetimes
                    .iter()
                    .map(|lt| MaybeStatic::NonStatic(*lt))
                    .collect();
                TypeInfo::fmt_generic_lifetimes(new_lts, self.lifetime_env)
            }
        }

        let type_name = self.formatter.fmt_symbol_name(self.id);
        let name = self.formatter.fmt_struct_abi_name(type_name.clone());
        // FIXME: Hacky, needs to be able to take into account namespaces
        self.imports
            .remove(&format!("{}::{name}", heck::AsSnakeCase(type_name.clone())));

        StructTemplate {
            type_name,
            methods,
            lib_name: self.lib_name,
            imports: self.imports,
            is_out,
            fields,
            lifetime_env,
            lifetimes,
        }
    }

    pub(super) fn generate_opaque(mut self, ty: &'tcx OpaqueDef) -> impl TypeTemplate<'tcx> {
        #[derive(Template)]
        #[template(path = "shared_rust/opaque.rs.jinja", escape = "none")]
        struct OpaqueTemplate<'a> {
            type_name: Cow<'a, str>,
            methods: Vec<FunctionInfo<'a>>,
            lib_name: String,
            imports: BTreeSet<String>,
            dtor_abi: Cow<'a, str>,
            lifetime_env: &'a LifetimeEnv,
        }

        let type_name = self.formatter.fmt_symbol_name(self.id);

        let dtor_abi = ty
            .attrs
            .abi_rename
            .apply(format!("{}_destroy", ty.name.as_str()).into());

        let methods = FunctionInfo::gen_function_block(&mut self, ty.methods.iter());

        impl<'a> TypeTemplate<'a> for OpaqueTemplate<'a> {
            fn render(&self) -> askama::Result<String> {
                askama::Template::render(self)
            }
            fn imports(&mut self) -> &mut BTreeSet<String> {
                &mut self.imports
            }
            fn mod_name(&self) -> String {
                self.type_name.clone().into()
            }
            fn generic_lifetimes(&self) -> String {
                TypeInfo::fmt_generic_lifetimes(
                    self.lifetime_env.lifetimes().lifetimes().collect(),
                    self.lifetime_env,
                )
            }
        }

        OpaqueTemplate {
            type_name,
            methods,
            lib_name: self.lib_name,
            imports: self.imports,
            dtor_abi,
            lifetime_env: &ty.lifetimes,
        }
    }

    pub(super) fn generate_enum(mut self, ty: &'tcx EnumDef) -> impl TypeTemplate<'tcx> {
        #[derive(Template)]
        #[template(path = "shared_rust/enum.rs.jinja", escape = "none")]
        struct EnumTemplate<'a> {
            type_name: Cow<'a, str>,
            methods: Vec<FunctionInfo<'a>>,
            lib_name: String,
            imports: BTreeSet<String>,
            variants: Vec<Cow<'a, str>>,
        }

        let methods = FunctionInfo::gen_function_block(&mut self, ty.methods.iter());

        let variants = ty
            .variants
            .iter()
            .map(|v| self.formatter.fmt_enum_variant_name(v))
            .collect();

        impl<'a> TypeTemplate<'a> for EnumTemplate<'a> {
            fn render(&self) -> askama::Result<String> {
                askama::Template::render(self)
            }
            fn imports(&mut self) -> &mut BTreeSet<String> {
                &mut self.imports
            }
            fn mod_name(&self) -> String {
                self.type_name.clone().into()
            }
            fn generic_lifetimes(&self) -> String {
                String::new()
            }
        }

        EnumTemplate {
            type_name: self.formatter.fmt_symbol_name(self.id),
            methods,
            lib_name: self.lib_name,
            imports: self.imports,
            variants,
        }
    }

    /// Generate [`TypeInfo`] for a given [`Type`]. See [`TypeInfo`] for more information.
    /// Mutable since [`FileGenContext`] may need to update imports depending on the type.
    pub(super) fn gen_type_info<P: TyPosition>(&'rcx mut self, ty: &Type<P>) -> TypeInfo<'tcx> {
        match ty {
            Type::Primitive(p) => TypeInfo::new(self.formatter.fmt_primitive_name(*p).into()),
            Type::Struct(st) => {
                let st_name = self.formatter.fmt_symbol_name(st.id().into());
                self.add_import(st_name.clone().into());

                TypeInfo {
                    borrow: st.owner(),
                    name: st_name,
                    generic_lifetimes: st.lifetimes().lifetimes().collect(),
                    wrapped: TypeInfoWrapper::None,
                }
            }
            Type::Enum(e) => {
                let type_id: TypeId = e.tcx_id.into();
                let enum_name = self.formatter.fmt_symbol_name(type_id.into());
                self.imports.insert(enum_name.clone().into());

                TypeInfo::new(enum_name)
            }
            Type::Opaque(op) => {
                let type_id: TypeId = op.tcx_id.into();
                let op_name = self.formatter.fmt_symbol_name(type_id.into());
                self.add_import(op_name.clone().into());

                TypeInfo {
                    name: op_name,
                    borrow: op.owner.get_borrow(),
                    generic_lifetimes: op.lifetimes.lifetimes().collect(),
                    wrapped: match (op.is_owned(), op.is_optional()) {
                        (true, true) => super::formatter::TypeInfoWrapper::BoxedOptional,
                        (true, false) => super::formatter::TypeInfoWrapper::Boxed,
                        (false, true) => super::formatter::TypeInfoWrapper::Optional,
                        (false, false) => super::formatter::TypeInfoWrapper::None,
                    },
                }
            }
            Type::DiplomatOption(op) => {
                let info = self.gen_type_info(op);
                TypeInfo {
                    name: format!("Option<{}>", info.name).into(),
                    generic_lifetimes: info.generic_lifetimes,
                    borrow: info.borrow,
                    // TODO: I don't think this always needs to be wrapped the same way:
                    wrapped: info.wrapped,
                }
            }
            Type::Slice(sl) => {
                let (borrow, type_name) = match sl {
                    Slice::Primitive(b, p) => {
                        let name = format!("[{}]", self.formatter.fmt_primitive_name(*p));
                        (b, name)
                    }
                    Slice::Struct(b, str) => {
                        let name = self.formatter.fmt_symbol_name(str.id().into());
                        let name = format!("[{name}]");
                        (b, name)
                    }
                    Slice::Str(lt, enc) => {
                        // TODO: I don't think this always needs to be wrapped the same way:
                        let name = match enc {
                            StringEncoding::Utf8 if lt.is_none() => "String",
                            StringEncoding::Utf8 => "str",
                            StringEncoding::UnvalidatedUtf8 => "[u8]",
                            StringEncoding::UnvalidatedUtf16 => "[u16]",
                            _ => panic!("Unknown encoding {enc:?}"),
                        }
                        .into();

                        match lt {
                            Some(lt) => (&MaybeOwn::from_immutable_lifetime(*lt), name),
                            None => (&MaybeOwn::Own, format!("{name}")),
                        }
                    }
                    Slice::Strs(enc) => {
                        // We perform borrows (without updating TypeInfo to reflect them), because we don't have borrow information, so we assume that this `is all elided:
                        let inner = match enc {
                            StringEncoding::Utf8 => "String",
                            StringEncoding::UnvalidatedUtf8 => "&[u8]",
                            StringEncoding::UnvalidatedUtf16 => "&[u16]",
                            _ => panic!("Unknown encoding {enc:?}"),
                        };
                        return TypeInfo::new(format!("&[{inner}]").into());
                    }
                    _ => unreachable!("Unknown slice type {sl:?}"),
                };

                TypeInfo {
                    name: type_name.into(),
                    generic_lifetimes: Vec::new(),
                    borrow: *borrow,
                    wrapped: if borrow.is_owned() {
                        TypeInfoWrapper::Boxed
                    } else {
                        TypeInfoWrapper::None
                    },
                }
            }
            _ => todo!(),
        }
    }

    /// TODO: Add namespacing params.
    pub(super) fn add_import(&mut self, import: String) {
        self.imports.insert(import.into());
    }
}
