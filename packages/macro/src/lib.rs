extern crate proc_macro;

mod action_parser;
mod async_action;
mod cli;
mod config;
mod subcommand;
mod ui_action;
mod ui_state;

use crate::{
    async_action::AsyncActionParser, cli::CliParser, config::ConfigParser,
    subcommand::SubcommandParser, ui_action::UiActionParser, ui_state::UiStateParser,
};
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemEnum, ItemImpl, ItemStruct};

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

#[proc_macro_derive(SubcommandPlugin)]
pub fn subcommand_plugin(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemEnum);

    SubcommandParser::new(input).parse().gen()
}

#[proc_macro_attribute]
pub fn config_plugin(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(tokens as ItemStruct);

    ConfigParser::new(attrs, input).parse().gen()
}

#[proc_macro_attribute]
pub fn async_action(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemImpl);

    AsyncActionParser::new(input).parse().gen()
}
