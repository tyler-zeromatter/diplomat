use diplomat_core::ast;
use syn::{parse2, parse_macro_input, ItemMod};
use quote::{quote, ToTokens};

fn gen_bridge(mut input : ItemMod) -> ItemMod {
    let module = ast::Module::from_syn(&input, true);
    
    let (brace, mut new_contents) = input.content.unwrap();

    new_contents.push(parse2(quote! { use diplomat_runtime::*; }).unwrap());

    ItemMod {
        attrs: input.attrs,
        vis: input.vis,
        mod_token: input.mod_token,
        ident: input.ident,
        content: Some((brace, new_contents)),
        semi: input.semi,
        unsafety: None,
    }
}

#[proc_macro_attribute]
pub fn bridge(_attr: proc_macro::TokenStream, input : proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_bridge(parse_macro_input!(input)).to_token_stream().into()
}