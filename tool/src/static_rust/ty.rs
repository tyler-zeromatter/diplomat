use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir::{StructDef, TypeDef, TypeId};

use crate::static_rust::RustFormatter;

pub(super) struct TyGenContext<'tcx> {
    formatter : &'tcx RustFormatter<'tcx>,
    id: TypeId,
}

pub(super) trait TypeTemplate<'a> : Template {}

impl<'tcx, 'rcx> TyGenContext<'tcx> {
    pub(super) fn from_type<'a>(id : TypeId, ty : TypeDef<'a>, formatter : &'a RustFormatter) -> impl TypeTemplate<'a> {
        let ctx = TyGenContext {
            formatter,
            id,
        };
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
            struct_name : Cow<'a, str>
        }

        impl<'a> TypeTemplate<'a> for StructTemplate<'a> {}

        StructTemplate {
            struct_name: self.formatter.fmt_symbol_name(self.id.into())
        }
    }
}