use askama::Template;
use diplomat_core::{ast, hir::BackendAttrSupport};
use quote::{quote, ToTokens};
use std::{
    cell::RefCell, collections::{BTreeMap, BTreeSet}, env, fmt, io::Write, path::{Path, PathBuf}, process::{Command, Stdio}
};
use syn::{visit_mut::VisitMut, Ident};

use crate::{config::Config, ErrorStore, FileMap};

pub(crate) fn attr_support() -> BackendAttrSupport {
    BackendAttrSupport::all_true()
}

#[derive(Template)]
#[template(path = "rust/lib.rs.jinja", escape="none")]
struct RustModules {
    modules : RefCell<BTreeSet<String>>,
}

impl RustModules {
    fn add_module(&self, module : String) {
        self.modules.borrow_mut().insert(module);
    }
}

struct DiplomatBridgeMod<'tcx, 'ccx> {
    filename : String,
    lib_name : &'tcx str,
    entry: &'tcx Path,
    files: &'tcx FileMap,
    errors: &'tcx ErrorStore<'ccx, String>,
    module : Option<ast::Module>,
    modules : &'tcx RustModules,
    function_abis : BTreeMap<Ident, Ident>,
}

impl<'tcx, 'ccx> DiplomatBridgeMod<'tcx, 'ccx> {
    fn get_module(&mut self, i : &syn::ItemMod) {
        self.module = Some(ast::Module::from_syn(i, true));

        for (_, ty) in &self.module.as_ref().unwrap().declared_types {
            for m in ty.methods() {
                self.function_abis.insert(m.name.to_syn(), m.abi_name.to_syn());
            }
        }
    }

    fn parse_file(file : syn::File) {

    }
}

// VisitMut to strip out unwanted files when ultimately outputting to our binding directory.
impl<'tcx, 'ccx> VisitMut for DiplomatBridgeMod<'tcx, 'ccx> {
    fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
        if i.content.is_some() {
            // TODO: Remove MacroRules and MacroUse here, parse macro expressions here.
            // TODO: Remove `use::` expressions. We'll add those back in for any function definitions.
            let bridge = i.attrs.iter().find(|a| { a.meta == syn::parse_quote!(diplomat::bridge) });
            if bridge.is_some() {
                self.modules.add_module(self.filename.clone());
                self.get_module(i);

                Self::clear_attributes(&mut i.attrs);
                let lib_name = self.lib_name;
                i.attrs.push(syn::parse_quote!(#[diplomat_static_rust::bridge(lib_name = #lib_name)]));
                for item in &mut i.content.as_mut().unwrap().1 {
                    self.visit_item_mut(item);
                }
            } else {
                i.attrs.clear();
                // TODO: Get parent and remove the content.
                // Or maybe just have a more robust system for only adding #[diplomat::bridge].
                i.content = None;
            }
        } else {
            let try_mod_path = self.entry.join(format!("{}.rs", i.ident.to_string()));
            if try_mod_path.exists() {
                let src = std::fs::read_to_string(&try_mod_path);
                if let Err(e) = &src {
                    self.errors
                        .push_error(format!("Could not read file: {}", e.to_string()));
                }
                let file = syn::parse_file(&src.unwrap());

                if let Err(e) = &file {
                    self.errors
                        .push_error(format!("Could not parse file: {}", e.to_string()));
                }

                let mut module = DiplomatBridgeMod {
                    filename: i.ident.to_string(),
                    lib_name: self.lib_name,
                    entry: self.entry,
                    files: self.files,
                    errors: self.errors,
                    module: None,
                    modules: self.modules,
                    function_abis: BTreeMap::new(),
                };
                let mut file = file.unwrap();
                // TODO: Cyclical dependencies?
                module.visit_file_mut(&mut file);

                self.files.add_file(
                    try_mod_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    prettyplease::unparse(&file),
                );
            } else {
                self.errors
                    .push_error(format!("Could not find path {try_mod_path:?}"));
            }
        }
    }

    fn visit_impl_item_fn_mut(&mut self, i: &mut syn::ImplItemFn) {
        Self::clear_attributes(&mut i.attrs);
        let name = self.function_abis.get(&i.sig.ident).unwrap();
        let mut call : syn::ExprCall = syn::parse_quote!(#name());
        i.sig.inputs.iter().for_each(|p| {
            let out = match p {
                syn::FnArg::Receiver(r) => {
                    let self_token = r.self_token;
                    quote!(#self_token)
                }
                syn::FnArg::Typed(ty) => {
                    ty.pat.to_token_stream()
                }
            };
            call.args.push(syn::parse_quote!(#out));
         });
        i.block = syn::parse_quote!({ unsafe { #call } });
    }

    fn visit_item_enum_mut(&mut self, i: &mut syn::ItemEnum) {
        Self::clear_attributes(&mut i.attrs);
        i.attrs.push(syn::parse_quote!(#[repr(C)]));
    }

    fn visit_item_struct_mut(&mut self, i: &mut syn::ItemStruct) {
        // TODO: Do we have an Opaque? If so, we need to hide the type.
        Self::clear_attributes(&mut i.attrs);
        i.attrs.push(syn::parse_quote!(#[repr(C)]));
    }

    fn visit_attribute_mut(&mut self, i: &mut syn::Attribute) {
        // i.meta = syn::Meta::Path(syn::parse_quote!(test))
    }
}

impl<'tcx, 'ccx> DiplomatBridgeMod<'tcx, 'ccx> {
    // TODO: Support renames.
    fn clear_attributes(attributes : &mut Vec<syn::Attribute>) {
        attributes.retain(|a| {
            a.meta.path().segments.first().unwrap().ident.to_string() != String::from("diplomat")
        });
    }
}

// Important TODOs:
// Macros. Should use Diplomat's built-in macro parser (since that's what it's built for)
// Opaque conversions.
// Stripping out #[diplomat] attributes.
// Ignoring anything outside of #[diplomat::bridge] (and renaming #[diplomat::bridge] to #[diplomat::rust] or something like that)
// Improved attribute support (renames and disables mostly)
pub(crate) fn run<'tcx>(entry: &Path, config : Config) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let errors = ErrorStore::default();

    // We should have already read and parsed the file earlier:
    let src = std::fs::read_to_string(entry).unwrap();
    let mut res = syn::parse_file(&src).unwrap();

    let modules = RustModules {
        modules: RefCell::new(BTreeSet::new()),
    };

    let mut main = DiplomatBridgeMod {
        filename: "".to_string(),
        lib_name : &config.shared_config.lib_name.expect("Need a lib_name for Rust to link against."),
        // TODO: Better entry parsing.
        entry: entry.parent().unwrap_or(Path::new("./")),
        files: &files,
        errors: &errors,
        module: None,
        modules: &modules,
        function_abis: BTreeMap::new(),
    };
    main.visit_file_mut(&mut res);

    files.add_file("lib.rs".into(), modules.render().unwrap());

    (files, errors)
}
