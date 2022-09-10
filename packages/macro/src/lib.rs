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

#[proc_macro_derive(CliPlugin)]
pub fn cli_plugin(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    CliParser::new(input).parse().gen()
}

#[proc_macro_derive(Subcommand)]
pub fn subcommand(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemEnum);

    SubcommandParser::new(input).parse().gen()
}
