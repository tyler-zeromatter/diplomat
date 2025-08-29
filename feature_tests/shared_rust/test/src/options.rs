#[cfg(test)]
mod tests {
    use shared_rust_feature_test_bindings::{OptionEnum, OptionInputStruct, OptionOpaque};

    #[test]
    fn test_option_opaque() {
        let opaque = OptionOpaque::new(5).unwrap();
        opaque.assert_integer(5);

        let opaque = OptionOpaque::new_none();
        assert!(opaque.is_none());

        // TODO: Out structs are not done yet.
        // I think crate_vis will need to be removed from `TypeTemplate`, and instead the fields will have to be made private to anything outside of the crate.
        // Then we'll just need to make getter functions for all of the fields.
        // let str = OptionOpaque::new_struct();
        
        // let sn = OptionOpaque::new_struct_nones();
    }

    #[test]
    fn test_u8() {
        let maybe_u8 = OptionOpaque::accepts_option_u8(None, 123);
        assert!(maybe_u8.is_none());
        let maybe_u8 = OptionOpaque::accepts_option_u8(Some(47), 123);
        assert_eq!(maybe_u8, Some(47));
    }

    #[test]
    fn test_enum() {
        let enm = OptionOpaque::accepts_option_enum(None, 123);
        assert!(enm.is_none());
        let enm = OptionOpaque::accepts_option_enum(Some(OptionEnum::Foo), 123);
        assert!(matches!(enm, Some(OptionEnum::Foo)));
    }

    #[test]
    fn test_struct() {
        let strct = OptionOpaque::accepts_option_input_struct(None, 123);
        assert!(strct.is_none());
        let strct = OptionOpaque::accepts_option_input_struct(Some(OptionInputStruct {
            a: Some(7),
            b: None,
            c: Some(OptionEnum::Bar)
        }), 123).unwrap();
        assert_eq!(strct.a, Some(7));
        assert!(strct.b.is_none());
        assert!(matches!(strct.c, Some(OptionEnum::Bar)));

        // TODO: This is failing because structs need a `FromFFI` function (mostly to convert `DiplomatOption` to `Option` and for any other types that need a conversion).
        let strct = OptionOpaque::returns_option_input_struct();
        assert_eq!(strct.a, Some(6));
        assert!(strct.b.is_none());
        assert!(matches!(strct.c, Some(OptionEnum::Bar)));
    }
}