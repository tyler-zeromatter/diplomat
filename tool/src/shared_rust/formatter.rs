use std::borrow::Cow;

use diplomat_core::hir::{
    EnumVariant, Lifetime, LifetimeEnv, MaybeOwn, MaybeStatic, Mutability, PrimitiveType, SymbolId,
    TypeContext,
};
use itertools::Itertools;

use crate::shared_rust::func::ABITypeInfo;

#[derive(Default)]
pub enum TypeInfoWrapper {
    #[default]
    None,
    Boxed,
    BoxedOptional,
    Optional,
}

/// All information relevant to displaying a type in any position in Rust. This just includes the type name and generic/borrow information.
/// This is *NOT* meant to include conversion information or ABI-specific info. That should be handled mostly by the `func` module.
pub(super) struct TypeInfo<'a> {
    pub(super) name: Cow<'a, str>,
    pub(super) generic_lifetimes: Vec<MaybeStatic<Lifetime>>,
    pub(super) wrapped : TypeInfoWrapper,
    pub(super) borrow: MaybeOwn,
}

impl<'a> TypeInfo<'a> {
    pub(super) fn new(name: Cow<'a, str>) -> Self {
        Self {
            name,
            generic_lifetimes: Vec::new(),
            borrow: MaybeOwn::Own,
            wrapped: TypeInfoWrapper::None,
        }
    }

    pub(super) fn fmt_generic_lifetimes(
        generic_lifetimes: Vec<MaybeStatic<Lifetime>>,
        env: &LifetimeEnv,
    ) -> String {
        let generic_lifetimes: Vec<String> = generic_lifetimes
            .iter()
            .map(|lt| match lt {
                MaybeStatic::Static => "'static".into(),
                MaybeStatic::NonStatic(ns) => {
                    format!("'{}", env.fmt_lifetime(ns))
                },
            })
            .collect();

        let generic_lifetimes_string = generic_lifetimes.join(", ");

        if !generic_lifetimes.is_empty() {
            format!("<{generic_lifetimes_string}>")
        } else {
            "".into()
        }
    }

    pub(super) fn fmt_generic_bounded_lifetimes(
        generic_lifetimes: Vec<MaybeStatic<Lifetime>>,
        env: &LifetimeEnv,
    ) -> String {
        let generic_lifetimes: Vec<String> = generic_lifetimes
            .iter()
            .map(|lt| match lt {
                MaybeStatic::Static => "'static".into(),
                MaybeStatic::NonStatic(ns) => {
                    // TODO: Does this work okay?
                    let bounded_lts : Vec<Lifetime> = env.all_shorter_lifetimes(ns).filter(|l| {
                        l != ns
                    }).collect();
                    let bounded_str = bounded_lts.iter().map(|l| {
                        format!("'{}", env.fmt_lifetime(l))
                    }).join(" + ");
                    let bounded = if bounded_lts.len() > 0 {
                        format!(": {bounded_str}")
                    } else {
                        "".into()
                    };
                    format!("'{}{bounded}", env.fmt_lifetime(ns))
                },
            })
            .collect();

        let generic_lifetimes_string = generic_lifetimes.join(", ");

        if !generic_lifetimes.is_empty() {
            format!("<{generic_lifetimes_string}>")
        } else {
            "".into()
        }
    }

    /// Format a given Rust type name with the the following layout:
    /// WrapperBegin &'lifetime mut TypeName<GenericLifetimes> WrapperEnd
    pub(super) fn render(&self, env: &LifetimeEnv) -> String {
        self.render_with_override(env, &ABITypeInfo::default())
    }

    pub(super) fn render_with_override(&self, env: &LifetimeEnv, over: &ABITypeInfo) -> String {
        let borrow = over.borrow.unwrap_or(self.borrow);

        let maybe_borrow: Cow<'_, str> = match borrow {
            MaybeOwn::Own => "".into(),
            MaybeOwn::Borrow(b) => match b.lifetime {
                MaybeStatic::Static => "static".into(),
                MaybeStatic::NonStatic(ns) => env.fmt_lifetime(ns),
            },
        };
        let borrow_stmt = match borrow {
            MaybeOwn::Own => "".into(),
            // TODO: Would be nice to have a LifetimeEnv helper to avoid formatting anonymous lifetimes.
            MaybeOwn::Borrow(b) if b.mutability == Mutability::Mutable => {
                format!("&'{maybe_borrow} mut ")
            }
            _ => format!("&'{maybe_borrow} "),
        };

        let name = over.name.clone().unwrap_or(self.name.clone().into());

        let generic_lifetimes = Self::fmt_generic_lifetimes(over.generic_lifetimes.as_ref().unwrap_or(self.generic_lifetimes.as_ref()).clone(), env);

        let name = format!("{name}{generic_lifetimes}");
        let name_wrapped = match over.wrapped.as_ref().unwrap_or(&self.wrapped) {
            TypeInfoWrapper::None => name,
            TypeInfoWrapper::Boxed => format!("Box<{name}>"),
            TypeInfoWrapper::BoxedOptional => format!("Option<Box<{name}>>"),
            TypeInfoWrapper::Optional => format!("Option<{name}>"),
        };

        format!("{borrow_stmt}{name_wrapped}")
    }
}

pub(super) struct RustFormatter<'tcx> {
    pub(super) tcx: &'tcx TypeContext,
}

impl<'tcx> RustFormatter<'tcx> {
    pub(super) fn fmt_symbol_name(&self, id: SymbolId) -> Cow<'tcx, str> {
        match id {
            SymbolId::FunctionId(f) => self.tcx.resolve_function(f).name.as_str().into(),
            SymbolId::TypeId(ty) => {
                let resolved = self.tcx.resolve_type(ty);
                let name = resolved.name();
                resolved.attrs().rename.apply(name.as_str().into())
            }
            SymbolId::TraitId(tr) => self.tcx.resolve_trait(tr).name.as_str().into(),
            _ => panic!("Symbol {id:?} unrecognized."),
        }
    }

    pub(super) fn fmt_primitive_name(&self, primitive: PrimitiveType) -> &'static str {
        match primitive {
            PrimitiveType::Char => "diplomat_runtime::DiplomatChar",
            PrimitiveType::Byte => "u8",
            _ => primitive.as_str(),
        }
    }

    pub(super) fn fmt_enum_variant_name(&self, enum_variant: &'tcx EnumVariant) -> Cow<'tcx, str> {
        format!(
            "{} = {}",
            enum_variant
                .attrs
                .rename
                .apply(enum_variant.name.as_str().into()),
            enum_variant.discriminant
        )
        .into()
    }
}
