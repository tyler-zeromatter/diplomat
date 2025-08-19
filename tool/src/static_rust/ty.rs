use std::{borrow::Cow, collections::BTreeSet};

use askama::Template;
use diplomat_core::hir::{StructDef, StructPathLike, SymbolId, TyPosition, Type, TypeContext, TypeDef, TypeId};

use crate::{config::Config, static_rust::{imp::FunctionInfo, RustFormatter}};

pub(super) struct FileGenContext<'tcx> {
    formatter : &'tcx RustFormatter<'tcx>,
    tcx : &'tcx TypeContext,
    id: SymbolId,
    lib_name : String,
    imports : BTreeSet<String>,
}

pub(super) trait TypeTemplate<'a> : Template {}

impl<'tcx, 'rcx> FileGenContext<'tcx> {
    pub(super) fn from_type<'a>(config : &Config, id : TypeId, formatter : &'a RustFormatter, tcx : &'a TypeContext) -> impl TypeTemplate<'a> {
        let mut ctx = FileGenContext {
            formatter,
            id: id.into(),
            tcx,
            lib_name: config.shared_config.lib_name.clone().expect("Rust static backend needs lib_name to link against."),
            imports: BTreeSet::new(),
        };
        let ty = ctx.tcx.resolve_type(id);
        match ty {
            TypeDef::Struct(st) => {
                ctx.generate_struct(st)
            }
            _ => panic!("Unsupported HIR type {ty:?}")
        }
    }

    fn generate_struct(&'rcx mut self, ty : &'tcx StructDef) -> impl TypeTemplate<'tcx> {
        #[derive(Template)]
        #[template(path = "static_rust/base.rs.jinja", escape = "none")]
        struct StructTemplate<'a> {
            struct_name : Cow<'a, str>,
            methods : Vec<FunctionInfo<'a>>,
            lib_name: String,
        }

        let methods = FunctionInfo::gen_function_block(self, ty.methods.iter());

        impl<'a> TypeTemplate<'a> for StructTemplate<'a> {}

        StructTemplate {
            struct_name: self.formatter.fmt_symbol_name(self.id.into()),
            methods,
            lib_name: self.lib_name.clone()
        }
    }

    pub(super) fn gen_type_name<P: TyPosition>(&mut self, ty : &Type<P>) -> String {
        match ty {
            Type::Struct(st) => {
                let st_name : String = self.formatter.fmt_symbol_name(st.id().into()).into();
                self.imports.insert(st_name.clone());
                st_name
            }
            Type::Primitive(p) => {
                self.formatter.fmt_primitive_name(*p).into()
            }
            _ => "TODO()".into()
        }
    }
}