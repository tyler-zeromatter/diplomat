use std::{borrow::Cow, collections::BTreeSet};

use askama::Template;
use diplomat_core::hir::{StructDef, StructPathLike, SymbolId, TyPosition, Type, TypeContext, TypeDef, TypeId};

use crate::{config::Config, static_rust::{func::FunctionInfo, RustFormatter}};

pub(super) struct FileGenContext<'tcx> {
    formatter : &'tcx RustFormatter<'tcx>,
    tcx : &'tcx TypeContext,
    id: SymbolId,
    lib_name : String,
    imports : BTreeSet<String>,
}

pub(super) trait TypeTemplate<'a> : Template {
    fn imports(&mut self) -> &mut BTreeSet<String>;
    fn mod_name(&self) -> String;
}

impl<'tcx, 'rcx> FileGenContext<'tcx> {
    pub(super) fn from_type<'a>(config : &Config, id : TypeId, formatter : &'a RustFormatter, tcx : &'a TypeContext) -> impl TypeTemplate<'a> {
        let ctx = FileGenContext {
            formatter,
            id: id.into(),
            tcx,
            lib_name: config.shared_config.lib_name.clone().expect("Rust static backend needs lib_name to link against."),
            imports: BTreeSet::new(),
        };
        let ty = ctx.tcx.resolve_type(id);
        let mut template = match ty {
            TypeDef::Struct(st) => {
                ctx.generate_struct(st)
            }
            _ => panic!("Unsupported HIR type {ty:?}")
        };
        template.imports().remove::<str>(&formatter.fmt_symbol_name(id.into()));
        template
    }

    fn generate_struct(mut self, ty : &'tcx StructDef) -> impl TypeTemplate<'tcx> {
        #[derive(Template)]
        #[template(path = "static_rust/base.rs.jinja", escape = "none")]
        struct StructTemplate<'a> {
            struct_name : Cow<'a, str>,
            methods : Vec<FunctionInfo<'a>>,
            lib_name: String,
            imports : BTreeSet<String>,
        }

        let methods = FunctionInfo::gen_function_block(&mut self, ty.methods.iter());

        impl<'a> TypeTemplate<'a> for StructTemplate<'a> {
            fn imports(&mut self) -> &mut BTreeSet<String> {
                &mut self.imports
            }
            fn mod_name(&self) -> String {
                self.struct_name.clone().into()
            }
        }

        StructTemplate {
            struct_name: self.formatter.fmt_symbol_name(self.id.into()),
            methods,
            lib_name: self.lib_name,
            imports: self.imports
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
            _ => "TODO()".into()
        }
    }
}