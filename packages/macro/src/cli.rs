use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
// use std::str::FromStr;
use syn::{ItemEnum, ItemStruct};

pub struct CliParser;

impl From<ItemStruct> for CliParser {
    fn from(_input: ItemStruct) -> Self {
        Self {}
    }
}

impl CliParser {
    pub fn parse(&self) -> CliTokenStreams {
        CliTokenStreams {}
    }
}

pub struct CliTokenStreams {
    // clap_attr: Vec<TokenStream2>,
    // cli_struct: Vec<TokenStream2>,
}

impl CliTokenStreams {
    pub fn gen(&self) -> TokenStream {
        let gen = quote! {
            use ::clap::Parser;

            #[derive(Parser)]
            #[clap(author, version, about, long_about = None)]
            struct DipCli {
                #[clap(subcommand)]
                command: Commands,
            }

            pub struct CliPlugin;

            impl ::bevy::app::Plugin for CliPlugin {
                fn build(&self, app: &mut ::bevy::app::App) {
                    app.insert_resource(DipCli::parse())
                        .add_plugin(SubcommandPlugin);
                }
            }
        };

        gen.into()
    }
}

pub struct SubcommandParser;

impl SubcommandParser {
    pub fn new(_attr: TokenStream, _item: ItemEnum) -> Self {
        Self {}
    }

    pub fn parse(&self) -> SubcommandTokenStreams {
        SubcommandTokenStreams {}
    }
}

pub struct SubcommandTokenStreams {
    // commands: Vec<TokenStream2>,
}

impl SubcommandTokenStreams {
    pub fn gen(&self) -> TokenStream {
        let gen = quote! {
            pub struct Build;
            pub struct Clean;

            #[derive(clap::Subcommand)]
            enum Commands {
                Build,
                Clean,
            }

            pub struct SubcommandPlugin;

            impl ::bevy::app::Plugin for SubcommandPlugin {
                fn build(&self, app: &mut ::bevy::app::App) {
                    app.insert_resource(DipCli::parse())
                        .add_event::<Build>()
                        .add_event::<Clean>()
                        .set_runner(|app| Self::runner(app));
                }
            }

            impl SubcommandPlugin {
                fn runner(mut app: ::bevy::app::App) {
                    let cli = app.world.get_resource::<DipCli>().unwrap();

                    match cli.command {
                        Commands::Build => {
                            app.world
                                .get_resource_mut::<::bevy::ecs::event::Events<Build>>()
                                .unwrap()
                                .send(Build);
                        }
                        Commands::Clean => {
                            app.world
                                .get_resource_mut::<::bevy::ecs::event::Events<Clean>>()
                                .unwrap()
                                .send(Clean);
                        }
                    }

                    app.update();
                }
            }
        };

        gen.into()
    }
}
