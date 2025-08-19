use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir::{StructDef, TypeContext, TypeDef, TypeId};

use crate::{config::Config, static_rust::{imp::FunctionInfo, RustFormatter}};

pub(super) struct TyGenContext<'tcx> {
    formatter : &'tcx RustFormatter<'tcx>,
    tcx : &'tcx TypeContext,
    id: TypeId,
    lib_name : String,
}

pub(super) trait TypeTemplate<'a> : Template {}

impl<'tcx, 'rcx> TyGenContext<'tcx> {
    pub(super) fn from_type<'a>(config : &Config, id : TypeId, formatter : &'a RustFormatter, tcx : &'a TypeContext) -> impl TypeTemplate<'a> {
        let ctx = TyGenContext {
            formatter,
            id,
            tcx,
            lib_name: config.shared_config.lib_name.clone().expect("Rust static backend needs lib_name to link against."),
        };
        let ty = ctx.tcx.resolve_type(id);
        match ty {
            TypeDef::Struct(st) => {
                ctx.generate_struct(st)
            }
            _ => panic!("Unsupported HIR type {ty:?}")
        }
    }

    fn generate_struct(&'rcx self, ty : &'tcx StructDef) -> impl TypeTemplate<'tcx> {
        #[derive(Template)]
        #[template(path = "static_rust/base.rs.jinja", escape = "none")]
        struct StructTemplate<'a> {
            struct_name : Cow<'a, str>,
            methods : Vec<FunctionInfo<'a>>,
            lib_name: String,
        }

        let methods = FunctionInfo::gen_function_block(ty.methods.iter());

        impl<'a> TypeTemplate<'a> for StructTemplate<'a> {}

        StructTemplate {
            struct_name: self.formatter.fmt_symbol_name(self.id.into()),
            methods,
            lib_name: self.lib_name.clone()
        }
    }
}