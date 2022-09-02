extern crate proc_macro;

mod ui_action;
mod ui_action_creator;
mod ui_state;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, ItemImpl, ItemStruct, Token, Type};
use ui_action::UiActionParser;
use ui_action_creator::UiActionCreatorParser;
use ui_state::UiStateParser;

#[proc_macro_attribute]
pub fn ui_state(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemStruct);

    UiStateParser::from(input).parse().gen()
}

#[proc_macro_attribute]
pub fn ui_action(args: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemImpl);

    let args: Vec<Type> =
        parse_macro_input!(args with Punctuated::<Type, Token![,]>::parse_terminated)
            .into_iter()
            .collect();
    let ui_action = UiActionParser::from(args).parse().gen();
    let ui_action_creator = UiActionCreatorParser::from(input).parse().gen();

    let gen = quote! {
        #ui_action
        #ui_action_creator
    };

    gen.into()
}
