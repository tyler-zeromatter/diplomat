use askama::{DynTemplate, Template};
use diplomat_core::{ast::DocType::Enum, hir::{EnumDef, StructDef}};

use crate::c::ItemGenContext as CItemGenContext;

pub(super) struct ItemGenContext<'cx, 'tcx, 'header> {
    pub c : CItemGenContext<'cx, 'tcx, 'header>,
}

pub(super) trait SymbolTemplate : DynTemplate {
    fn file_base_name(&self) -> String;
}

impl<'cx, 'tcx, 'header> ItemGenContext<'cx, 'tcx, 'header> {
    pub fn gen_enum_def(&self, def : &EnumDef) -> impl SymbolTemplate {
        #[derive(Template)]
        #[template(path = "cpython/symbols/enum.c.jinja", escape = "none")]
        struct EnumTemplate {
            name : String,
        }

        impl SymbolTemplate for EnumTemplate {
            fn file_base_name(&self) -> String {
                self.name.clone()
            }
        }
        EnumTemplate {
            name: def.name.as_str().into(),
        }
    }

    pub fn gen_struct_def(&self, def : &StructDef) -> impl SymbolTemplate {
        #[derive(Template)]
        #[template(path = "cpython/symbols/struct.c.jinja", escape = "none")]
        struct StructTemplate {
            class_name : String
        }
        impl SymbolTemplate for StructTemplate {
            fn file_base_name(&self) -> String {
                self.class_name.clone()
            }
        }
        StructTemplate {
            class_name: def.name.as_str().to_string(),
        }
    }
}