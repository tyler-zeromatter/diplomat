fn main() {
    println!("cargo::rustc-link-search=../../../target/debug");
    println!("cargo::rustc-link-lib=somelib:diplomat_feature_tests.dll");
}