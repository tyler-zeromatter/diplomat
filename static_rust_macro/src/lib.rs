use diplomat_core::ast;
use syn::{parse::{Parse, Parser}, parse2, parse_macro_input, Attribute, ItemMod};
use quote::{quote, ToTokens};

fn gen_bridge(mut input : ItemMod, attribute : StaticRustAttr) -> ItemMod {
    let module = ast::Module::from_syn(&input, true);
    
    let (brace, mut new_contents) = input.content.unwrap();

    let mut extern_block : syn::ItemForeignMod = parse2(quote!{ extern "C" {} }).unwrap();
    
    let link_name = attribute.lib_name;
    extern_block.attrs.push(syn::parse_quote! { #[link(name=#link_name)] });
        
    for custom_type in module.declared_types.values() {
        custom_type.methods().iter().for_each(|m| {
            let abi_name = &m.abi_name;
            let mut extern_fn : syn::ForeignItemFn = syn::parse_quote! { fn #abi_name(); };
            if let Some(s) = &m.self_param {
                let type_ident = custom_type.name().to_syn();
                let is_borrowed = match &s.reference {
                    Some((_, mt)) if mt.is_mutable() => quote!(&mut),
                    Some(..) => quote!(&),
                    None => quote!()
                };
                extern_fn.sig.inputs.push(syn::parse_quote!(this : #is_borrowed #type_ident));
            }

            m.params.iter().enumerate().for_each(|(i, p)| { 
                let ty = p.ty.to_syn();
                let name = p.name.to_syn();
                extern_fn.sig.inputs.push(syn::parse_quote!( #name : #ty ));
             });

             if let Some(o) = &m.return_type {
                let out = o.to_syn();
                extern_fn.sig.output = syn::parse_quote!(-> #out);
             }
            extern_block.items.push(extern_fn.into());
        });
    }

    new_contents.push(syn::Item::ForeignMod(extern_block));

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

struct StaticRustAttr {
    lib_name : String,
}

impl Parse for StaticRustAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let eq = input.parse::<syn::Token![=]>()?;
        let lib_name = input.parse::<syn::LitStr>()?;
        Ok(Self {
            lib_name:  lib_name.value()
        })
    }
}

#[proc_macro_attribute]
pub fn bridge(attr: proc_macro::TokenStream, input : proc_macro::TokenStream) -> proc_macro::TokenStream {
    let p = syn::parse::<StaticRustAttr>(attr);
    
    if let Err(e) = &p {
        return e.to_compile_error().into();
    }
    
    gen_bridge(parse_macro_input!(input), p.unwrap()).to_token_stream().into()
}