extern crate proc_macro;

mod cli;
mod ui_action;
mod ui_state;

use cli::{CliParser, SubcommandParser};
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum, ItemImpl, ItemStruct};
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

#[proc_macro_attribute]
pub fn cli_plugin(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemStruct);

    CliParser::new(attr, input).parse().gen()
}

#[proc_macro_attribute]
pub fn cli_subcommand(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemEnum);

    SubcommandParser::new(attr, input).parse().gen()
}
