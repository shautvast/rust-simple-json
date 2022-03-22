use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    
    let DeriveInput { ident, .. } = input;

    let output = quote! {
        impl MyTrait for #ident {
            fn answer() -> i32 {
                42
            }
        }
    };
    output.into()
}
