use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Parser)]
pub fn parser(input: TokenStream) -> TokenStream {
    quote! {}.into()
}
