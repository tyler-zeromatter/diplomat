fn main() {
    // TODO: There has to be a better solution.
    // Copy the file over:
    // println!("cargo::rustc-link-search=../../../target/debug");
    if cfg!(windows) {
        std::fs::copy("../../../target/debug/diplomat_feature_tests.dll", "../target/debug/diplomat_feature_tests.dll").unwrap();
    } else {
        std::fs::copy("../../../target/debug/diplomat_feature_tests.so", "../target/debug/diplomat_feature_tests.so").unwrap();
    }
}