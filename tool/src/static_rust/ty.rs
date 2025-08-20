use std::{borrow::Cow, collections::BTreeSet};

use askama::Template;
use diplomat_core::hir::{EnumDef, OpaqueDef, StructDef, StructPathLike, SymbolId, TyPosition, Type, TypeContext, TypeDef, TypeId};

use crate::{config::Config, static_rust::{func::FunctionInfo, RustFormatter}};

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
        #[derive(Template)]
        #[template(path = "static_rust/struct.rs.jinja", escape = "none")]
        struct StructTemplate<'a> {
            type_name : Cow<'a, str>,
            methods : Vec<FunctionInfo<'a>>,
            lib_name: String,
            imports : BTreeSet<String>,
            is_out : bool,
            // TODO: Fields.
        }

        let methods = FunctionInfo::gen_function_block(&mut self, ty.methods.iter());

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
        }

        StructTemplate {
            type_name: self.formatter.fmt_symbol_name(self.id.into()),
            methods,
            lib_name: self.lib_name,
            imports: self.imports,
            is_out
        }
    }

    pub(super) fn generate_opaque(mut self, ty : &'tcx OpaqueDef) -> impl TypeTemplate<'tcx> {
        #[derive(Template)]
        #[template(path = "static_rust/opaque.rs.jinja", escape = "none")]
        struct OpaqueTemplate<'a> {
            type_name : Cow<'a, str>,
            methods : Vec<FunctionInfo<'a>>,
            lib_name: String,
            imports : BTreeSet<String>,
        }

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
        }

        OpaqueTemplate {
            type_name: self.formatter.fmt_symbol_name(self.id.into()),
            methods,
            lib_name: self.lib_name,
            imports: self.imports,
        }
    }

    pub(super) fn generate_enum(mut self, ty : &'tcx EnumDef) -> impl TypeTemplate<'tcx> {
        #[derive(Template)]
        #[template(path = "static_rust/enum.rs.jinja", escape = "none")]
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
        }

        EnumTemplate {
            type_name: self.formatter.fmt_symbol_name(self.id.into()),
            methods,
            lib_name: self.lib_name,
            imports: self.imports,
            variants,
        }
    }

    pub(super) fn gen_type_name<P: TyPosition>(&'rcx mut self, ty : &Type<P>) -> Cow<'tcx, str> {
        match ty {
            Type::Primitive(p) => {
                self.formatter.fmt_primitive_name(*p).into()
            }
            Type::Struct(st) => {
                let st_name = self.formatter.fmt_symbol_name(st.id().into());
                self.imports.insert(st_name.clone().into());
                st_name
            }
            Type::Enum(e) => {
                let type_id : TypeId = e.tcx_id.into();
                let enum_name = self.formatter.fmt_symbol_name(type_id.into());
                self.imports.insert(enum_name.clone().into());
                enum_name
            }
            Type::Opaque(op) => {
                let type_id : TypeId = op.tcx_id.into();
                let op_name = self.formatter.fmt_symbol_name(type_id.into());
                self.imports.insert(op_name.clone().into());
                op_name
            }
            Type::DiplomatOption(op) => {
                format!("Option<{}>", self.gen_type_name(op)).into()
            }
            _ => "TODO()".into()
        }
    }

    pub(super) fn gen_abi_type_name<P: TyPosition>(&'rcx mut self, ty : &Type<P>) -> Option<Cow<'tcx, str>> {
        match ty {
            Type::DiplomatOption(op) => {
                Some(format!("DiplomatOption<{}>", self.gen_type_name(ty)).into())
            }
            _ => None
        }
    }
}