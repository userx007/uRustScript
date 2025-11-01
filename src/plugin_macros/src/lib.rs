use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

#[proc_macro_attribute]
pub fn register_commands(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let self_ty = &input.self_ty;

    // Collect all methods starting with "cmd_"
    let mut cmds = vec![];
    for item in &input.items {
        if let syn::ImplItem::Fn(meth) = item {
            let name = meth.sig.ident.to_string();
            if name.starts_with("cmd_") {
                let cmd_name = name.trim_start_matches("cmd_").to_uppercase();
                let ident = &meth.sig.ident;
                cmds.push(quote! {
                    self.commands.register(#cmd_name, #self_ty::#ident);
                });
            }
        }
    }

    let expanded = quote! {
        #input

        impl #self_ty {
            pub fn register_commands(&mut self) {
                #(#cmds)*
            }
        }
    };

    TokenStream::from(expanded)
}
