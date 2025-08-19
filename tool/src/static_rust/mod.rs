use askama::Template;
use diplomat_core::hir::{BackendAttrSupport, TypeContext};

use crate::{static_rust::ty::TyGenContext, ErrorStore, FileMap};

mod ty;
mod imp;

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut support = BackendAttrSupport::default();
    support.option = true;
    
    support
}

pub(crate) fn run<'tcx>(tcx : &'tcx TypeContext) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let errors = ErrorStore::default();

    for (id, ty) in tcx.all_types() {
        match ty {
            crate::hir::TypeDef::Struct(st) => files.add_file(format!("{}.rs", ty.name().as_str().into()), TyGenContext::from_type(ty).render().unwrap()),
            _ => {}
        }
    }

    for (id, func) in tcx.all_free_functions() {

    }

    (files, errors)
}