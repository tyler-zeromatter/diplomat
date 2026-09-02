mod formatter;

use askama::Template;
use diplomat_core::hir::{BackendAttrSupport, DocsUrlGenerator, TypeContext};
use serde::{Deserialize, Serialize};

use crate::{ErrorStore, FileMap, c::ItemGenContext as CItemGenContext, cpython::formatter::PyFormatter};


pub fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct CPythonConfig {

}

struct ItemGenContext<'cx, 'tcx, 'header> {
    pub c : CItemGenContext<'cx, 'tcx, 'header>,
}

pub fn run<'tcx>(tcx : &'tcx TypeContext, config : &crate::Config, docs_url_gen : &'tcx DocsUrlGenerator) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let errors = ErrorStore::default();
    let formatter = PyFormatter::new(tcx, config, docs_url_gen);

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