pub struct MixinTest;

#[diplomat::macro_rules]
#[macro_export]
macro_rules! mixin_macro {
    () => {
        impl MixinTest {
            pub fn hello(w : &mut DiplomatWrite) -> {
                write!(w, "Hello!").unwrap();
            }
        }
    };
}