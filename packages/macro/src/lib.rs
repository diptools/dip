extern crate proc_macro;

mod ui_action;
mod ui_state;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl, ItemStruct};
use ui_action::UiActionParser;
use ui_state::UiStateParser;

#[proc_macro_attribute]
pub fn ui_state(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemStruct);

    UiStateParser::from(input).parse().gen()
}

#[proc_macro_attribute]
pub fn ui_action(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemImpl);

    UiActionParser::from(input).parse().gen()
}
