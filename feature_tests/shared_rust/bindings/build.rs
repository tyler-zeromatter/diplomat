fn main() {
    println!("cargo::rustc-link-lib=dylib=somelib:diplomat_feature_tests.dll");
    // We run `cargo test` from feature_tests/shared_rust, so we account for that path here:
    println!(r"cargo:rustc-link-search=dylib=../../target/debug");
    // if cfg!(windows) {
    //     println!("cargo::rustc-link-lib=somelib:diplomat_feature_tests.dll");
    // } else {
    //     println!("cargo::rustc-link-lib=somelib:diplomat_feature_tests.dll");
    // }
}