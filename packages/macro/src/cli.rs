use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemEnum, ItemStruct};

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

        for cmd in &self.commands_enum.variants {
            let ident = &cmd.ident;

            tokens.events.push(quote! { pub struct #ident; });
            tokens.add_events.push(quote! { .add_event::<#ident>() });
            tokens.handlers.push(quote! {
                Commands::#ident => {
                    app.world
                        .get_resource_mut::<::bevy::ecs::event::Events<#ident>>()
                        .unwrap()
                        .send(#ident);
                }
            });
        }

        tokens
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

                    match cli.command {
                        #(#handlers)*
                    }

                    app.update();
                }
            }
        };

        gen.into()
    }
}
