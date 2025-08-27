use std::{borrow::Cow, collections::BTreeSet};

use askama::Template;
use diplomat_core::hir::{Borrow, EnumDef, Lifetime, LifetimeEnv, MaybeOwn, MaybeStatic, Mutability, OpaqueDef, OpaqueOwner, OpaquePath, Slice, StringEncoding, StructDef, StructPathLike, SymbolId, TyPosition, Type, TypeContext, TypeDef, TypeId};
use itertools::Itertools;

use crate::{config::Config, shared_rust::{func::FunctionInfo, RustFormatter}};

pub(super) struct FileGenContext<'tcx> {
    pub(super) formatter : &'tcx RustFormatter<'tcx>,
    tcx : &'tcx TypeContext,
    id: SymbolId,
    lib_name : String,
    imports : BTreeSet<String>,
}

pub(super) trait TypeTemplate<'a> {
    fn render(&self) -> askama::Result<String>;
    fn imports(&mut self) -> &mut BTreeSet<String>;
    fn mod_name(&self) -> String;
    fn crate_vis(&self) -> Option<String>;
}

/// All information relevant to displaying a type in any position in Rust. This just includes the type name and generic/borrow information.
pub(super) struct TypeInfo<'a> {
    pub(super) name : Cow<'a, str>,
    pub(super) generic_lifetimes : Vec<MaybeStatic<Lifetime>>,
    pub(super) borrow : MaybeOwn,
}

impl<'a> TypeInfo<'a> {
    pub(super) fn new(name : Cow<'a, str>) -> Self {
        Self {
            name,
            generic_lifetimes: Vec::new(),
            borrow: MaybeOwn::Own,
        }
    }

    pub(super) fn render(&self, env : &LifetimeEnv) -> String {
        let maybe_borrow = match self.borrow {
            MaybeOwn::Own => "".into(),
            MaybeOwn::Borrow(b) => match b.lifetime {
                MaybeStatic::Static => "static".into(),
                MaybeStatic::NonStatic(ns) => env.fmt_lifetime(ns),
            }
        };
        let borrow_stmt = match self.borrow {
            MaybeOwn::Own => "".into(),
            MaybeOwn::Borrow(b) if b.mutability == Mutability::Mutable => format!("&'{maybe_borrow} mut "),
            _ => format!("&'{maybe_borrow} ")
        };

        let generic_lifetimes : Vec<String> = self.generic_lifetimes.iter().map(|lt| {
            match lt {
                MaybeStatic::Static => "'static".into(),
                MaybeStatic::NonStatic(ns) => format!("'{}", env.fmt_lifetime(ns))
            }
        }).collect();

        let generic_lifetimes_string = generic_lifetimes.join(", ");

        format!("{borrow_stmt}{}{}", self.name, if generic_lifetimes.len() > 0 {
            format!("<{generic_lifetimes_string}>")
        } else {
            "".into()
        })
    }
}

impl<'tcx, 'rcx> FileGenContext<'tcx> {
    pub(super) fn from_type<'a>(config : &Config, id : TypeId, formatter : &'a RustFormatter, tcx : &'a TypeContext) -> FileGenContext<'a> {
        FileGenContext {
            formatter,
            id: id.into(),
            tcx,
            lib_name: config.shared_config.lib_name.clone().expect("Rust static backend needs lib_name to link against."),
            imports: BTreeSet::new(),
        }
    }

    pub(super) fn generate_struct<P: TyPosition>(mut self, ty : &'tcx StructDef<P>, is_out : bool) -> impl TypeTemplate<'tcx> {
        struct FieldInfo<'a> {
            type_info : TypeInfo<'a>,
            name : Cow<'a, str>,
            generic_lifetimes : Vec<MaybeStatic<Lifetime>>,
        }

        impl<'a> FieldInfo<'a> {
            fn generic_lifetimes(&self, env : &LifetimeEnv) -> Vec<String> {
                self.generic_lifetimes.iter().map(|l| {
                    match l {
                        MaybeStatic::Static => "static".to_string(),
                        MaybeStatic::NonStatic(ns) => env.fmt_lifetime(ns).to_string()
                    }
                }).collect()
            }
        }

        #[derive(Template)]
        #[template(path = "shared_rust/struct.rs.jinja", escape = "none")]
        struct StructTemplate<'a> {
            type_name : Cow<'a, str>,
            methods : Vec<FunctionInfo<'a>>,
            lib_name: String,
            imports : BTreeSet<String>,
            is_out : bool,
            fields : Vec<FieldInfo<'a>>,
            lifetime_env : &'a LifetimeEnv,
            lifetimes : Vec<String>,
        }

        let methods = FunctionInfo::gen_function_block(&mut self, ty.methods.iter());

        let lifetime_env = &ty.lifetimes;
        let lifetimes = lifetime_env.all_lifetimes().map(|lt| {
            lifetime_env.fmt_lifetime(lt).into()
        });

        let fields = ty.fields.iter().map(|f| {
            let generic_lifetimes = f.ty.lifetimes();

            FieldInfo {
                type_info: self.gen_type_info(&f.ty),
                name: f.name.as_str().into(),
                generic_lifetimes: generic_lifetimes.collect(),
            }
        }).collect();

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
            fn crate_vis(&self) -> Option<String> {
                if self.is_out {
                    Some("(crate)".into())
                } else {
                    None
                }
            }
        }

        StructTemplate {
            type_name: self.formatter.fmt_symbol_name(self.id.into()),
            methods,
            lib_name: self.lib_name,
            imports: self.imports,
            is_out,
            fields,
            lifetime_env,
            lifetimes: lifetimes.collect(),
        }
    }

    pub(super) fn generate_opaque(mut self, ty : &'tcx OpaqueDef) -> impl TypeTemplate<'tcx> {
        #[derive(Template)]
        #[template(path = "shared_rust/opaque.rs.jinja", escape = "none")]
        struct OpaqueTemplate<'a> {
            type_name : Cow<'a, str>,
            methods : Vec<FunctionInfo<'a>>,
            lib_name: String,
            imports : BTreeSet<String>,
            dtor_abi : Cow<'a, str>,
        }

        let type_name = self.formatter.fmt_symbol_name(self.id.into());

        let dtor_abi = ty.attrs.abi_rename.apply(format!("{}_destroy", ty.name.as_str()).into());

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
            fn crate_vis(&self) -> Option<String> {
                None
            }
        }

        OpaqueTemplate {
            type_name,
            methods,
            lib_name: self.lib_name,
            imports: self.imports,
            dtor_abi
        }
    }

    pub(super) fn generate_enum(mut self, ty : &'tcx EnumDef) -> impl TypeTemplate<'tcx> {
        #[derive(Template)]
        #[template(path = "shared_rust/enum.rs.jinja", escape = "none")]
        struct EnumTemplate<'a> {
            type_name : Cow<'a, str>,
            methods : Vec<FunctionInfo<'a>>,
            lib_name: String,
            imports : BTreeSet<String>,
            variants : Vec<Cow<'a, str>>,
        }

        let methods = FunctionInfo::gen_function_block(&mut self, ty.methods.iter());

        let variants = ty.variants.iter().map(|v| {
            self.formatter.fmt_enum_variant_name(v)
        }).collect();

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
            fn crate_vis(&self) -> Option<String> {
                None
            }
        }

        EnumTemplate {
            type_name: self.formatter.fmt_symbol_name(self.id.into()),
            methods,
            lib_name: self.lib_name,
            imports: self.imports,
            variants,
        }
    }

    pub(super) fn gen_type_info<P: TyPosition>(&'rcx mut self, ty : &Type<P>) -> TypeInfo<'tcx> {
        match ty {
            Type::Primitive(p) => {
                TypeInfo::new(self.formatter.fmt_primitive_name(*p).into())
            }
            Type::Struct(st) => {
                let st_name = self.formatter.fmt_symbol_name(st.id().into());
                self.imports.insert(st_name.clone().into());
                
                TypeInfo {
                    borrow: st.owner(),
                    name: st_name,
                    generic_lifetimes: st.lifetimes().lifetimes().collect(),
                }
            }
            Type::Enum(e) => {
                let type_id : TypeId = e.tcx_id.into();
                let enum_name = self.formatter.fmt_symbol_name(type_id.into());
                self.imports.insert(enum_name.clone().into());
                
                TypeInfo::new(enum_name)
            }
            Type::Opaque(op) => {
                let type_id : TypeId = op.tcx_id.into();
                let op_name = self.formatter.fmt_symbol_name(type_id.into());
                self.imports.insert(op_name.clone().into());

                let op_name = if op.is_owned() {
                    format!("Box<{op_name}>").into()
                } else {
                    // TODO: Lifetimes
                    format!("{op_name}").into()
                };

                TypeInfo {
                    name: if op.is_optional() {
                        format!("Option<{op_name}>").into()
                    } else {
                        op_name
                    },
                    borrow: op.owner.get_borrow(),
                    generic_lifetimes: op.lifetimes.lifetimes().collect(),
                }
            }
            Type::DiplomatOption(op) => {
                let info = self.gen_type_info(op);
                TypeInfo {
                    name: format!("Option<{}>", info.name).into(),
                    generic_lifetimes: info.generic_lifetimes,
                    borrow: info.borrow
                }
            }
            Type::Slice(sl) => {
                let (borrow, type_name) = match sl {
                    Slice::Primitive(b, p) => {
                        let name = if b.is_owned() {
                            format!("Box<[{}]>", self.formatter.fmt_primitive_name(*p)).into()
                        } else {
                            format!("[{}]", self.formatter.fmt_primitive_name(*p)).into()
                        };
                        (b, name)
                    }
                    Slice::Struct(b, str) => {
                        let name = self.formatter.fmt_symbol_name(str.id().into());
                        let name = if b.is_owned() {
                            format!("Box<[{name}]>").into()
                        } else {
                            format!("[{name}]").into()
                        };
                        (b, name)
                    }
                    Slice::Str(lt, enc) => {
                        let name = match enc {
                            StringEncoding::Utf8 => "String",
                            StringEncoding::UnvalidatedUtf8 => "[u8]",
                            StringEncoding::UnvalidatedUtf16 => "[u16]",
                            _ => panic!("Unknown encoding {enc:?}")
                        }.into();

                        match lt {
                            Some(lt) => (&MaybeOwn::from_immutable_lifetime(lt.clone()), name),
                            None => (&MaybeOwn::Own, format!("Box<{name}>")),
                            _ => (&MaybeOwn::Own, name)
                        }
                    }
                    Slice::Strs(enc) => {
                        // TODO: Need to read encoding properly.
                        return TypeInfo::new("&[String]".into());
                    }
                    _ => unreachable!("Unknown slice type {sl:?}"),
                };

                TypeInfo { 
                    name: type_name.into(),
                    generic_lifetimes: Vec::new(),
                    borrow: *borrow,
                }
            }
            _ => TypeInfo::new("TODO()".into())
        }
    } 

    pub(super) fn gen_abi_type_name<P: TyPosition>(&'rcx mut self, ty : &Type<P>) -> Option<Cow<'tcx, str>> {
        match ty {
            Type::DiplomatOption(op) => {
                let regular_type = self.gen_type_info(op).name;
                Some(format!("diplomat_runtime::DiplomatOption<{}>", self.gen_abi_type_name(op).unwrap_or(regular_type)).into())
            }
            Type::Slice(sl) => match sl {
                // TODO: Lifetimes.
                Slice::Str(..) => Some(format!("diplomat_runtime::DiplomatStrSlice").into()),
                _ => None
            }
            _ => None
        }
    }
}