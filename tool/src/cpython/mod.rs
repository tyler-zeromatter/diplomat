mod formatter;
mod gen;

use askama::{DynTemplate, Template};
use diplomat_core::hir::{BackendAttrSupport, DocsUrlGenerator, TypeContext, TypeDef};
use serde::{Deserialize, Serialize};

use crate::cpython::gen::SymbolTemplate;
use crate::{ErrorStore, FileMap, c::{self, ItemGenContext as CItemGenContext}, cpython::{formatter::PyFormatter, r#gen::ItemGenContext}};


pub fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct CPythonConfig {

}

pub fn run<'tcx>(tcx : &'tcx TypeContext, config : &crate::Config, docs_url_gen : &'tcx DocsUrlGenerator) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let errors = ErrorStore::default();
    let formatter = PyFormatter::new(tcx, config, docs_url_gen);
    let (c_files, c_errors) = c::run(tcx, config, docs_url_gen);
    
    files.files.borrow_mut().extend(
        c_files
            .files
            .take()
            .into_iter()
            .map(|(k, v)| (format!("include/{k}"), v)),
    );
    errors.errors.borrow_mut().extend(c_errors.errors.take());

    for (id, ty) in tcx.all_types() {
        let context = ItemGenContext {
            c : CItemGenContext {
                tcx,
                formatter: &formatter.c,
                errors: &errors,
                is_for_cpp: false,
                decl_header_path: &formatter.c.fmt_decl_header_path(id.into()),
                impl_header_path: &formatter.c.fmt_impl_header_path(id.into())
            }
        };

        let template : &dyn SymbolTemplate = match ty {
            TypeDef::Enum(e) => {
                &context.gen_enum_def(e)
            }
            TypeDef::Struct(st) => {
                &context.gen_struct_def(st)
            }
            _ => { continue; } // TODO
        };
        files.add_file(format!("{}_binding.c", template.file_base_name()), template.dyn_render().unwrap());


    }

    #[derive(Template)]
    #[template(path = "cpython/runtime.c.jinja", escape = "none")]
    struct RuntimeTemplate {
        lib_name : String,
    }

    let runtime = RuntimeTemplate {
        lib_name: config.shared_config.lib_name.clone().expect("CPython backend expects lib_name to be set."),
    };

    files.add_file("diplomat_runtime.c".into(), runtime.render().expect("Could not render template"));

    #[derive(Template)]
    #[template(path = "cpython/common.h.jinja", escape = "none")]
    struct RuntimeHeaderTemplate {

    }

    let runtime_header = RuntimeHeaderTemplate {};

    files.add_file("diplomat_runtime_common.h".into(), runtime_header.render().expect("Could not render header"));

    (files, errors)
}