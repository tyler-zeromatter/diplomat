use std::borrow::Cow;

use diplomat_core::hir::{DocsUrlGenerator, EnumVariant, SymbolId, TypeContext, TypeId};

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

    pub fn fmt_type_name(&self, id : TypeId) -> Cow<'tcx, str> {
        let resolved = self.c.tcx().resolve_type(id);
        let name = resolved.name().as_str().into();
        heck::AsUpperCamelCase(resolved.attrs().rename.apply(name)).to_string().into()
    }

    pub fn fmt_enum_variant(&self, variant : &'tcx EnumVariant) -> Cow<'tcx, str> {
        variant.attrs.rename.apply(variant.name.as_str().into())
    }
}