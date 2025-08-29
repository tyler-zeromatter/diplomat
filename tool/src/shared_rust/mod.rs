use std::collections::BTreeSet;

use askama::Template;
use diplomat_core::hir::{BackendAttrSupport, TypeContext, TypeDef};

use crate::{
    config::Config,
    shared_rust::{
        formatter::RustFormatter,
        ty::{FileGenContext, TypeTemplate},
    },
    ErrorStore, FileMap,
};

mod formatter;
mod func;
mod ty;

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut support = BackendAttrSupport::default();
    support.option = true;
    // support.struct_refs = true;
    // Support can be added gradually. I think the main goal should be getting a robust test suite of what's already been generated so far.

    support
}

pub(crate) fn run<'tcx>(
    tcx: &'tcx TypeContext,
    config: Config,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let errors = ErrorStore::default();

    let formatter = RustFormatter { tcx };

    #[derive(PartialEq, PartialOrd, Eq, Ord)]
    /// A `mod type;` `pub use type;` statement.
    struct ModImport {
        mod_name: String,
        type_name: String,
        /// TODO: This is only for OutStructs, should be removed since you can't access a pub(crate) struct.
        /// Everything should just be `pub`, and we should add getters to the OutStructs.
        vis: Option<String>,
    }

    #[derive(Template)]
    #[template(path = "shared_rust/lib.rs.jinja", escape = "none")]
    struct LibFile {
        mods: BTreeSet<ModImport>,
    }

    let mut lib = LibFile {
        mods: BTreeSet::new(),
    };

    for (id, ty) in tcx.all_types() {
        let name = formatter.fmt_symbol_name(id.into());

        let ctx = FileGenContext::from_type(&config, id, &formatter, tcx);
        let template: &mut dyn TypeTemplate = match ty {
            TypeDef::Struct(st) => &mut ctx.generate_struct(st, false),
            TypeDef::OutStruct(st) => &mut ctx.generate_struct(st, true),
            TypeDef::Opaque(op) => &mut ctx.generate_opaque(op),
            TypeDef::Enum(e) => &mut ctx.generate_enum(e),
            _ => unreachable!("Unsupported HIR type {ty:?}"),
        };

        template
            .imports()
            .remove::<str>(&formatter.fmt_symbol_name(id.into()));

        let mod_name = heck::AsSnakeCase(name).to_string();
        lib.mods.insert(ModImport {
            mod_name: mod_name.clone(),
            type_name: template.mod_name(),
            vis: template.crate_vis(),
        });
        files.add_file(format!("{}.rs", mod_name), template.render().unwrap())
    }

    for (id, func) in tcx.all_free_functions() {
        // See [`FunctionInfo::gen_function_block`].
    }

    files.add_file("lib.rs".into(), lib.render().unwrap());

    (files, errors)
}
