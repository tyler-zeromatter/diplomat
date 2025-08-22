use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    if cfg!(windows) {
        println!("cargo::rustc-link-lib=dylib=somelib:diplomat_feature_tests.dll");
        println!(r"cargo:rustc-link-search=dylib=../../target/debug");
    } else {
        println!(r"cargo:rustc-link-search={out_dir}");
        println!("cargo::rustc-link-lib=somelib:diplomat_feature_tests");
    }

    if cfg!(windows) {
        std::fs::copy("../../../target/debug/diplomat_feature_tests.dll", format!("../target/debug/diplomat_feature_tests.dll")).unwrap();
    } else {
        std::fs::copy("../../../target/debug/libdiplomat_feature_tests.so", format!("{out_dir}/libdiplomat_feature_tests.so")).unwrap();
    }
    // We run `cargo test` from feature_tests/shared_rust, so we account for that path here:
}