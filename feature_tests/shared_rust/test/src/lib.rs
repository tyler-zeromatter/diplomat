#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::MyEnum;
    #[test]
    fn test_enum() {
        let a = match MyEnum::get_a() {
            MyEnum::A => "A",
            MyEnum::B => "B",
            MyEnum::C => "C",
            MyEnum::D => "D",
            MyEnum::F => "F",
            MyEnum::E => "E"
        };
        assert_eq!(a, "A");
        let e = MyEnum::B;
        assert_eq!(e.into_value(), -1);
    }
}