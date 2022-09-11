use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
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

        CliTokenStreams {
            cli_name: quote! { #cli_name },
        }
    }
}

#[derive(Default)]
pub struct CliTokenStreams {
    cli_name: TokenStream2,
}

impl CliTokenStreams {
    pub fn gen(&self) -> TokenStream {
        let Self { cli_name } = self;
        let gen = quote! {
            use ::clap::Parser;

            pub struct CliPlugin;

            impl ::bevy::app::Plugin for CliPlugin {
                fn build(&self, app: &mut ::bevy::app::App) {
                    app.insert_resource(#cli_name::parse())
                        .add_plugin(SubcommandPlugin);
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

        for v in &self.commands_enum.variants {
            tokens.events.push(Self::event(&v));
            tokens.add_events.push(Self::add_event(&v));
            tokens.handlers.push(Self::handler(&v));
        }

        tokens
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

        match &v.fields {
            Fields::Named(_) | Fields::Unnamed(_) => {
                quote! { .add_event::<#ident>() }
            }
            Fields::Unit => {
                quote! { .add_event::<#ident>() }
            }
        }
    }

    fn handler(v: &Variant) -> TokenStream2 {
        let ident = &v.ident;

        match &v.fields {
            Fields::Named(fields) => {
                let mut names = vec![];
                for f in &fields.named {
                    names.push(f.ident.clone().unwrap());
                }

                quote! {
                    Commands::#ident { #(#names)*, } => {
                        app.world
                            .get_resource_mut::<::bevy::ecs::event::Events<#ident>>()
                            .unwrap().send(#ident { #(#names)*, });
                    }
                }
            }
            Fields::Unnamed(_f) => {
                quote! {
                    Commands::#ident(x) => {
                        app.world
                            .get_resource_mut::<::bevy::ecs::event::Events<#ident>>()
                            .unwrap().send(x.clone());
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    Commands::#ident => {
                        app.world
                            .get_resource_mut::<::bevy::ecs::event::Events<#ident>>()
                            .unwrap().send(#ident);
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SubcommandTokenStreams {
    events: Vec<TokenStream2>,
    add_events: Vec<TokenStream2>,
    handlers: Vec<TokenStream2>,
}

impl SubcommandTokenStreams {
    pub fn gen(&self) -> TokenStream {
        let Self {
            events,
            add_events,
            handlers,
        } = self;

        let gen = quote! {
            #(#events)*

            pub struct SubcommandPlugin;

            impl ::bevy::app::Plugin for SubcommandPlugin {
                fn build(&self, app: &mut ::bevy::app::App) {
                    app.insert_resource(DipCli::parse())
                        #(#add_events)*
                        .set_runner(|app| Self::runner(app));
                }
            }

            impl SubcommandPlugin {
                fn runner(mut app: ::bevy::app::App) {
                    let cli = app.world.get_resource::<DipCli>().unwrap();

                    match cli.command.clone() {
                        #(#handlers)*
                    }

                    app.update();
                }
            }
        };

        gen.into()
    }
}
