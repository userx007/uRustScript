use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

/// Automatically registers all impl functions as commands
#[proc_macro_attribute]
pub fn plugin_commands(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let mut commands = Vec::new();

    for item in &input.items {
        if let syn::ImplItem::Fn(meth) = item {
            let name = &meth.sig.ident;
            commands.push(quote! {
                self.commands.insert(
                    stringify!(#name).to_string(),
                    Box::new(|s: &mut _ , args: &str| s.#name(args))
                );
            });
        }
    }

    let original = quote! { #input };
    let gen = quote! {
        #original

        impl UtilsPlugin {
            pub fn register_commands(&mut self) {
                #(#commands)*
            }
        }
    };

    gen.into()
}
