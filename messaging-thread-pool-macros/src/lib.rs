use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn pool_item(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    
    // For now, just return the input unchanged
    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}
