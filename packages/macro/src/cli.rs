use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use std::str::FromStr;
use syn::{Fields, ItemEnum, ItemStruct, Variant};

pub struct CliParser {
    cli_struct: ItemStruct,
}

impl CliParser {
    pub fn new(cli_struct: ItemStruct) -> Self {
        Self { cli_struct }
    }

    pub fn parse(&self) -> CliTokenStreams {
        let cli_name = &self.cli_struct.ident;

        let mut subcommand_resource = quote! {};
        let mut subcommand_plugin = quote! {};

        for f in self.cli_struct.fields.iter() {
            for a in f.attrs.iter() {
                for t in a.tokens.clone().into_iter() {
                    match t {
                        TokenTree::Group(g) => {
                            for s in g.stream() {
                                match s {
                                    TokenTree::Ident(ident) => {
                                        if ident.to_string() == "subcommand" {
                                            let subcommand_name = f.ident.as_ref().unwrap();

                                            subcommand_resource = quote! { .insert_resource(cli.#subcommand_name.clone()) };
                                            subcommand_plugin =
                                                quote! { .add_plugin(SubcommandPlugin) };
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        CliTokenStreams {
            cli_name: quote! { #cli_name },
            subcommand_resource,
            subcommand_plugin,
        }
    }
}

#[derive(Default)]
pub struct CliTokenStreams {
    cli_name: TokenStream2,
    subcommand_resource: TokenStream2,
    subcommand_plugin: TokenStream2,
}

impl CliTokenStreams {
    pub fn gen(&self) -> TokenStream {
        let Self {
            cli_name,
            subcommand_resource,
            subcommand_plugin,
        } = self;

        let gen = quote! {
            use ::clap::Parser;

            pub struct CliPlugin;

            impl ::bevy::app::Plugin for CliPlugin {
                fn build(&self, app: &mut ::bevy::app::App) {
                    let cli = #cli_name::parse();

                    app.add_plugin(::dip::core::schedule::UiSchedulePlugin)
                        #subcommand_plugin
                        #subcommand_resource
                        .insert_resource(cli)
                        .set_runner(|mut app| {
                            app.update();
                        });
                }
            }
        };

        gen.into()
    }
}

pub struct SubcommandParser {
    commands_enum: ItemEnum,
}

impl SubcommandParser {
    pub fn new(commands_enum: ItemEnum) -> Self {
        Self { commands_enum }
    }

    pub fn parse(&self) -> SubcommandTokenStreams {
        let mut tokens = SubcommandTokenStreams::default();
        tokens.subcommand_ty_name = self.subcommand_ty_name();

        for v in &self.commands_enum.variants {
            tokens.events.push(Self::event(&v));
            tokens.add_events.push(Self::add_event(&v));
            tokens.event_readers.push(Self::event_reader(&v));
            tokens.handlers.push(Self::handler(&v));
        }

        tokens
    }

    fn subcommand_ty_name(&self) -> TokenStream2 {
        let ident = &self.commands_enum.ident;
        quote! { #ident }
    }

    fn event(v: &Variant) -> TokenStream2 {
        let ident = &v.ident;

        match &v.fields {
            Fields::Named(f) => {
                quote! {
                    #[derive(Debug)]
                    pub struct #ident #f
                }
            }
            Fields::Unnamed(f) => {
                let ty = &f.unnamed.first().unwrap().ty;
                quote! { type #ident = #ty; }
            }
            Fields::Unit => {
                let ident = &v.ident;
                quote! { pub struct #ident; }
            }
        }
    }

    fn add_event(v: &Variant) -> TokenStream2 {
        let ident = &v.ident;

        quote! { .add_event::<#ident>() }
    }

    fn event_reader(v: &Variant) -> TokenStream2 {
        let ident = &v.ident;
        let event_name_snake =
            TokenStream2::from_str(&ident.to_string().to_case(Case::Snake)).unwrap();

        quote! { mut #event_name_snake: ::bevy::ecs::event::EventWriter<#ident>, }
    }

    fn handler(v: &Variant) -> TokenStream2 {
        let ident = &v.ident;
        let event_name_snake =
            TokenStream2::from_str(&ident.to_string().to_case(Case::Snake)).unwrap();

        match &v.fields {
            Fields::Named(fields) => {
                let mut names = vec![];
                for f in &fields.named {
                    names.push(f.ident.clone().unwrap());
                }

                quote! {
                    Commands::#ident { #(#names)*, } => {
                        #event_name_snake.send(#ident { #(#names)*, });
                    }
                }
            }
            Fields::Unnamed(_f) => {
                quote! {
                    Commands::#ident(x) => {
                        #event_name_snake.send(x.clone());
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    Commands::#ident => {
                        #event_name_snake.send(#ident);
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SubcommandTokenStreams {
    subcommand_ty_name: TokenStream2,
    events: Vec<TokenStream2>,
    add_events: Vec<TokenStream2>,
    event_readers: Vec<TokenStream2>,
    handlers: Vec<TokenStream2>,
}

impl SubcommandTokenStreams {
    pub fn gen(&self) -> TokenStream {
        let Self {
            subcommand_ty_name,
            events,
            add_events,
            event_readers,
            handlers,
        } = self;

        let gen = quote! {
            #(#events)*

            pub struct SubcommandPlugin;

            impl ::bevy::app::Plugin for SubcommandPlugin {
                fn build(&self, app: &mut ::bevy::app::App) {
                    app #(#add_events)*
                        .add_system_to_stage(::dip::core::schedule::UiStage::Action, handle_subcommand);
                }
            }

            fn handle_subcommand(
                subcommand: Res<#subcommand_ty_name>,
                #(#event_readers)*
            ) {
                match subcommand.clone() {
                    #(#handlers)*
                }
            }
        };

        gen.into()
    }
}
