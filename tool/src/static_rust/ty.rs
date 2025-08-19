use askama::Template;
use diplomat_core::hir::{StructDef, TypeDef};

pub(super) struct TyGenContext {

}

pub(super) trait TypeTemplate<'a> : Template {}

impl TyGenContext {
    pub(super) fn from_type(ty : TypeDef) -> impl TypeTemplate {
        match ty {
            TypeDef::Struct(st) => {
                Self::generate_struct(st)
            }
            _ => panic!("Unsupported HIR type {ty:?}")
        }
    }

    fn generate_struct(ty : &StructDef) -> impl TypeTemplate<'_> {
        #[derive(Template)]
        #[template(path = "static_rust/base.rs.jinja", escape = "none")]
        struct StructTemplate<'a> {
            struct_name : &'a str
        }

        impl<'a> TypeTemplate<'a> for StructTemplate<'a> {}

        StructTemplate {
            struct_name: ty.name.as_str()
        }
    }
}