use std::borrow::Cow;

use diplomat_core::hir::{DocsUrlGenerator, EnumVariant, SymbolId, TypeContext};

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

    pub fn fmt_enum_variant(&self, variant : &'tcx EnumVariant) -> Cow<'tcx, str> {
        variant.attrs.rename.apply(variant.name.as_str().into())
    }
}