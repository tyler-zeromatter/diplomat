fn main() {
    println!("cargo::rustc-link-search=../../../target/debug");
    if cfg!(windows) {
        println!("cargo::rustc-link-lib=somelib:diplomat_feature_tests.dll");
    } else {
        println!("cargo::rustc-link-lib=somelib:diplomat_feature_tests");
    }
}