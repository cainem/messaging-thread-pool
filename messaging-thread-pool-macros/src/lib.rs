mod generation;
mod parsing;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl};

#[proc_macro_attribute]
pub fn pool_item(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    generation::generate_pool_item_impl(input).into()
}
