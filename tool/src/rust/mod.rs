use diplomat_core::hir::BackendAttrSupport;
use quote::ToTokens;
use std::{
    env, fmt,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use syn::visit_mut::VisitMut;

use crate::{ErrorStore, FileMap};

pub(crate) fn attr_support() -> BackendAttrSupport {
    BackendAttrSupport::all_true()
}

struct DiplomatBridgeFiles<'tcx, 'ccx> {
    entry: &'tcx Path,
    files: &'tcx FileMap,
    errors: &'tcx ErrorStore<'ccx, String>,
    rust_fmt: PathBuf,
}

// VisitMut to strip out unwanted files when ultimately outputting to our binding directory.
impl<'tcx, 'ccx> VisitMut for DiplomatBridgeFiles<'tcx, 'ccx> {
    fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
        if let Some((_, items)) = &mut i.content {
            for item in items {
                self.visit_item_mut(item);
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

                let mut file = file.unwrap();
                self.visit_file_mut(&mut file);

                // TODO: Will this work everywhere?
                let run = Command::new(self.rust_fmt.clone())
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn();
                if let Err(e) = &run {
                    self.errors
                        .push_error(format!("Could not run rustfmt: {e}"));
                    return;
                }
                let mut run = run.unwrap();
                let stdin = run.stdin.take();

                if let None = stdin {
                    self.errors
                        .push_error(format!("Could not get stdin for process: {run:?}"));
                    return;
                }

                let err = stdin
                    .unwrap()
                    .write_all(file.to_token_stream().to_string().as_bytes());
                if let Err(e) = err {
                    self.errors
                        .push_error(format!("Could not write file for formatting: {e}"));
                    return;
                }

                let out = run.wait_with_output();

                if let Err(e) = out {
                    self.errors
                        .push_error(format!("Could not get stdout for process: {e}"));
                    return;
                }

                self.files.add_file(
                    try_mod_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    String::from_utf8(out.unwrap().stdout).unwrap(),
                );
            } else {
                self.errors
                    .push_error(format!("Could not find path {try_mod_path:?}"));
            }
        }
    }

    fn visit_attribute_mut(&mut self, i: &mut syn::Attribute) {
        i.meta = syn::Meta::Path(syn::parse_quote!(test))
    }
}

pub(crate) fn run<'tcx>(entry: &Path) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let errors = ErrorStore::default();

    // We should have already read and parsed the file earlier:
    let src = std::fs::read_to_string(entry).unwrap();
    let mut res = syn::parse_file(&src).unwrap();

    // Like cargo expand, we just call rustfmt:
    let rust_fmt = match env::var_os("RUSTFMT") {
        Some(which) if !which.is_empty() => Some(PathBuf::from(which)),
        None => toolchain_find::find_installed_component("rustfmt"),
        _ => None,
    };

    let rust_fmt = if let Some(rust_fmt) = rust_fmt {
        rust_fmt
    } else {
        errors.push_error(
            "Could not find rustfmt command, needed for pretty-printing files.".to_string(),
        );
        return (files, errors);
    };

    let mut main = DiplomatBridgeFiles {
        // TODO: Better entry parsing.
        entry: entry.parent().unwrap_or(Path::new("./")),
        files: &files,
        errors: &errors,
        rust_fmt,
    };
    main.visit_file_mut(&mut res);

    (files, errors)
}
