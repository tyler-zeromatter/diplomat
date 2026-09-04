use diplomat_core::hir::{DocsUrlGenerator, SymbolId, TypeContext};

use crate::c::CFormatter;

pub(crate) struct PyFormatter<'tcx> {
    pub c : CFormatter<'tcx>,
    docs_url_gen : &'tcx DocsUrlGenerator,
}

impl<'tcx> PyFormatter<'tcx> {
    pub fn new(
        tcx: &'tcx TypeContext,
        config_for_c: &crate::Config,
        docs_url_gen: &'tcx DocsUrlGenerator,
    ) -> Self {
        Self {
            c: CFormatter::new(tcx, false, config_for_c, docs_url_gen),
            docs_url_gen,
        }
    }
}