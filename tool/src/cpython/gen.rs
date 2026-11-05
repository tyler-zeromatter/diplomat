use std::borrow::Cow;

use askama::{DynTemplate, Template};
use diplomat_core::{ast::DocType::Enum, hir::{EnumDef, InputOnly, IntType, OutputOnly, PrimitiveType, ReturnType, StructDef, SuccessType, TyPosition, Type}};

use crate::{c::{Header, ItemGenContext as CItemGenContext}, cpython::formatter::PyFormatter};

pub(super) struct ItemGenContext<'cx, 'tcx, 'header> {
    pub c : CItemGenContext<'cx, 'tcx, 'header>,
    pub fmt : &'cx PyFormatter<'tcx>,
}

pub(super) trait SymbolTemplate {
    fn file_base_name(&self) -> String;
    fn render_impl(&self) -> String;
    fn render_header(&self) -> String;
}

struct NamedType<'tcx, P: TyPosition> {
    name : Cow<'tcx, str>,
    c_type : Cow<'tcx, str>,
    ty : &'tcx Type<P>,
}

struct MethodInfo<'tcx> {
    name : Cow<'tcx, str>,
    c_name : Cow<'tcx, str>,
    inner : &'tcx diplomat_core::hir::Method,
    params : Vec<NamedType<'tcx, InputOnly>>,
    c_return : Option<Cow<'tcx, str>>,
    return_convert : Option<Cow<'tcx, str>>,
    is_write : bool,
}

impl<'cx, 'tcx, 'header> ItemGenContext<'cx, 'tcx, 'header> {
    pub fn gen_enum_def(&self, def : &'tcx EnumDef) -> impl SymbolTemplate + use<'tcx, 'cx> {
        #[derive(Template)]
        #[template(path = "cpython/symbols/enum.c.jinja", escape = "none", blocks=["header", "definition"])]
        struct EnumTemplate<'tcx> {
            abi_name : &'tcx str,
            name : String,
            def : &'tcx EnumDef,
            fmt : &'tcx PyFormatter<'tcx>,
            methods : Vec<MethodInfo<'tcx>>,
        }

        impl<'tcx> SymbolTemplate for EnumTemplate<'tcx> {
            fn file_base_name(&self) -> String {
                self.abi_name.into()
            }

            fn render_impl(&self) -> String {
                self.as_definition().render().unwrap()
            }
            fn render_header(&self) -> String {
                self.as_header().render().unwrap()
            }
        }
        let methods = def.methods.iter().filter_map(|m| {
            if m.attrs.disable {
                return None;
            }
            let params = m.params.iter().map(|p| {
                NamedType {
                    ty: &p.ty,
                    name: p.name.as_str().into(),
                    c_type: self.c.gen_ty_name(&p.ty, &mut Header::default()),
                }
            }).collect();

            let c_return = match &m.output {
                ReturnType::Infallible(SuccessType::Unit | SuccessType::Write(..)) => None,
                ReturnType::Infallible(SuccessType::OutType(o)) => Some(self.c.gen_ty_name(&o, &mut Header::default())),
                ReturnType::Fallible(ref ok, _) | ReturnType::Nullable(ref ok) => {
                    let err = if let ReturnType::Fallible(_, Some(ref e)) = m.output {
                        Some(e)
                    } else {
                        None
                    };

                    let ok_ty = match ok {
                        SuccessType::Write(..) | SuccessType::Unit => None,
                        SuccessType::OutType(o) => Some(o),
                        _ => unreachable!("Unknown AST/HIR variant"),
                    };

                    let res = self.c.gen_result_ty(m.abi_name.as_str(), ok_ty, err, &mut Header::default());
                    Some(res.into())
                }
                _ => unreachable!("unknown AST/HIR variant"),
            };

            let return_convert = self.convert_return_type(&m.output);

            Some(MethodInfo {
                name: m.name.as_str().into(),
                c_name : m.abi_name.as_str().into(),
                inner: m,
                params,
                c_return,
                return_convert,
                is_write: m.output.is_write(),
            })
        }).collect();

        EnumTemplate {
            abi_name: def.name.as_str(),
            name: heck::AsUpperCamelCase(def.name.as_str()).to_string(),
            def,
            fmt: self.fmt,
            methods,
        }
    }

    pub fn gen_struct_def(&self, def : &StructDef) -> impl SymbolTemplate {
        #[derive(Template)]
        #[template(path = "cpython/symbols/struct.c.jinja", escape = "none", blocks=["header", "definition"])]
        struct StructTemplate {
            class_name : String
        }
        impl SymbolTemplate for StructTemplate {
            fn file_base_name(&self) -> String {
                self.class_name.clone()
            }
            fn render_impl(&self) -> String {
                self.as_definition().render().unwrap()
            }
            fn render_header(&self) -> String {
                self.as_header().render().unwrap()
            }
        }
        StructTemplate {
            class_name: def.name.as_str().to_string(),
        }
    }

    fn convert_return_type(&self, ret : &ReturnType) -> Option<Cow<'tcx, str>> {
        match ret {
            ReturnType::Infallible(SuccessType::OutType(o)) => {
                self.convert_out_type(o)
            },
            // TODO:
            _ => None,
        }
    }

    fn convert_out_type(&self, out : &Type<OutputOnly>) -> Option<Cow<'tcx, str>> {
        match out {
            Type::Primitive(p) => match p {
                PrimitiveType::Bool => Some("Diplomat_bool_to_py".into()),
                PrimitiveType::Int(i) => match i {
                    IntType::I8 => Some("Diplomat_long_to_py".into()),
                    // TODO:
                    _ => None,
                }
                // TODO:
                _ => None,
            }
            Type::Enum(e) => {
                let en = self.c.tcx.resolve_enum(e.tcx_id);
                let abi_name = en.name.as_str();
                Some(format!("{abi_name}_to_py").into())
            }
            // TODO:
            _ => None,
        }
    }
}