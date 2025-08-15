#[proc_macro_attribute]
pub fn bridge(_attr: proc_macro::TokenStream, input : proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}