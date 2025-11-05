use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemImpl};

#[proc_macro_attribute]
pub fn plugin_commands(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = &input.self_ty;

    let mut method_names = Vec::new();
    let mut command_inserts = Vec::new();

    for impl_item in &input.items {
        if let ImplItem::Fn(meth) = impl_item {
            let name_ident = &meth.sig.ident;
            let name_str = name_ident.to_string();

            // HashMap insertion
            command_inserts.push(quote! {
                self.commands.insert(
                    #name_str.to_string(),
                    Box::new(|s: &mut _, args: &str| s.#name_ident(args))
                );
            });

            // method names for command_names()
            method_names.push(quote! { #name_str });
        }
    }

    let gen_command_names = quote! {
        impl #struct_name {
            pub fn command_names(&self) -> Vec<&'static str> {
                vec![#(#method_names),*]
            }

            pub fn register_commands(&mut self) {
                #(#command_inserts)*
            }
        }
    };

    let expanded = quote! {
        #input
        #gen_command_names
    };

    TokenStream::from(expanded)
}
