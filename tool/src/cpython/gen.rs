use std::borrow::Cow;

use askama::{DynTemplate, Template};
use diplomat_core::{ast::DocType::Enum, hir::{EnumDef, StructDef}};

use crate::{c::ItemGenContext as CItemGenContext, cpython::formatter::PyFormatter};

pub(super) struct ItemGenContext<'cx, 'tcx, 'header> {
    pub c : CItemGenContext<'cx, 'tcx, 'header>,
    pub fmt : &'cx PyFormatter<'tcx>,
}

pub(super) trait SymbolTemplate : DynTemplate {
    fn file_base_name(&self) -> String;
}

impl<'cx, 'tcx, 'header> ItemGenContext<'cx, 'tcx, 'header> {
    pub fn gen_enum_def(&self, def : &'tcx EnumDef) -> impl SymbolTemplate + use<'tcx, 'cx> {
        #[derive(Template)]
        #[template(path = "cpython/symbols/enum.c.jinja", escape = "none")]
        struct EnumTemplate<'tcx> {
            abi_name : &'tcx str,
            name : String,
            def : &'tcx EnumDef,
            fmt : &'tcx PyFormatter<'tcx>,
        }

        impl<'tcx> SymbolTemplate for EnumTemplate<'tcx> {
            fn file_base_name(&self) -> String {
                self.abi_name.into()
            }
        }
        EnumTemplate {
            abi_name: def.name.as_str(),
            name: heck::AsUpperCamelCase(def.name.as_str()).to_string(),
            def,
            fmt: self.fmt
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