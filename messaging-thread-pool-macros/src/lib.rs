mod generation;
mod parsing;

use parsing::PoolItemArgs;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl};

#[proc_macro_attribute]
pub fn pool_item(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as PoolItemArgs);
    let input = parse_macro_input!(item as ItemImpl);
    generation::generate_pool_item_impl(input, args).into()
}
