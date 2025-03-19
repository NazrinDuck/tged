extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

///
///
/// pos: (u16, u16),
/// height: u16,
/// width: u16,
#[proc_macro_attribute]
pub fn view(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
