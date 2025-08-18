fn main() {
    // Copy the DLL over:
    std::fs::copy("../../../target/debug/diplomat_feature_tests.dll", "./target/debug/diplomat_feature_tests.dll");
}