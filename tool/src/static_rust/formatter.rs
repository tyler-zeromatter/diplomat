use std::borrow::Cow;

use diplomat_core::hir::{PrimitiveType, SymbolId, TypeContext};

pub(super) struct RustFormatter<'tcx> {
    pub(super) tcx : &'tcx TypeContext,
}

impl<'tcx> RustFormatter<'tcx> {
    pub(super) fn fmt_symbol_name(&self, id : SymbolId) -> Cow<'tcx, str> {
        match id {
            SymbolId::FunctionId(f) => {
                self.tcx.resolve_function(f).name.as_str().into()
            }
            SymbolId::TypeId(ty) => {
                let resolved = self.tcx.resolve_type(ty);
                let name = resolved.name();
                resolved.attrs().rename.apply(name.as_str().into())
            }
            SymbolId::TraitId(tr) => {
                self.tcx.resolve_trait(tr).name.as_str().into()
            }
            _ => panic!("Symbol {id:?} unrecognized.")
        }
    }

    pub(super) fn fmt_primitive_name(&self, primitive : PrimitiveType) -> &'static str {
        primitive.as_str()
    }
}