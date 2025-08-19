use askama::Template;
use diplomat_core::hir::{BackendAttrSupport, TypeContext};

use crate::{config::Config, static_rust::{formatter::RustFormatter, ty::FileGenContext}, ErrorStore, FileMap};

mod ty;
mod imp;
mod formatter;

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut support = BackendAttrSupport::default();
    support.option = true;
    
    support
}

pub(crate) fn run<'tcx>(tcx : &'tcx TypeContext, config : Config) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let errors = ErrorStore::default();

    let formatter = RustFormatter {
        tcx,
    };

    for (id, ty) in tcx.all_types() {
        let name = formatter.fmt_symbol_name(id.into());
        match ty {
            crate::hir::TypeDef::Struct(st) => files.add_file(format!("{}.rs", name), FileGenContext::from_type(&config, id, &formatter, tcx).render().unwrap()),
            _ => {}
        }
    }

    for (id, func) in tcx.all_free_functions() {

    }

    (files, errors)
}