extern crate proc_macro;

mod action_parser;
mod cli;
mod config;
mod subcommand;
mod ui_state;

use crate::{
    action_parser::ActionParser, cli::CliParser, config::ConfigParser,
    subcommand::SubcommandParser, ui_state::UiStateParser,
};
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum, ItemImpl, ItemStruct};

#[proc_macro_attribute]
pub fn ui_state(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemStruct);

    UiStateParser::from(input).parse().gen()
}

#[proc_macro_attribute]
pub fn ui_action(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemImpl);

    ActionParser::ui_action(input).parse().gen()
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

#[proc_macro_derive(ConfigPlugin)]
pub fn config_plugin(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemStruct);

    ConfigParser::new(input).parse().gen()
}

#[proc_macro_attribute]
pub fn async_action(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemImpl);

    ActionParser::async_action(input).parse().gen()
}
