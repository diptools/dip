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
    // commands: Vec<TokenStream2>,
}

impl CliTokenStreams {
    pub fn gen(&self) -> TokenStream {
        let gen = quote! {
            use clap::Parser;

            #[derive(Parser)]
            #[clap(author, version, about, long_about = None)]
            struct DipCli {
                #[clap(subcommand)]
                command: Commands,
            }

            #[derive(clap::Subcommand)]
            enum Commands {
                Build,
                Clean,
            }

            pub struct Build;
            pub struct Clean;

            pub struct CliPlugin;

            impl dip::bevy::app::Plugin for CliPlugin {
                fn build(&self, app: &mut dip::bevy::app::App) {
                    app.insert_resource(DipCli::parse())
                        .add_event::<Build>()
                        .add_event::<Clean>()
                        .set_runner(|app| Self::runner(app));
                }
            }

            impl CliPlugin {
                fn runner(mut app: dip::bevy::app::App) {
                    let cli = app.world.get_resource::<DipCli>().unwrap();

                    match cli.command {
                        Commands::Build => {
                            app.world
                                .get_resource_mut::<dip::bevy::ecs::event::Events<Build>>()
                                .unwrap()
                                .send(Build);
                        }
                        Commands::Clean => {
                            app.world
                                .get_resource_mut::<dip::bevy::ecs::event::Events<Clean>>()
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

pub struct SubCommandParser;

impl From<ItemEnum> for SubCommandParser {
    fn from(_input: ItemEnum) -> Self {
        Self {}
    }
}

impl SubCommandParser {
    pub fn parse(&self) -> SubCommandTokenStreams {
        SubCommandTokenStreams {}
    }
}

pub struct SubCommandTokenStreams {
    // commands: Vec<TokenStream2>,
}

impl SubCommandTokenStreams {
    pub fn gen(&self) -> TokenStream {
        let gen = quote! {
            // pub struct Build;
            // pub struct Clean;

            // pub struct CliPlugin;

            // impl dip::bevy::app::Plugin for CliPlugin {
            //     fn build(&self, app: &mut dip::bevy::app::App) {
            //         app.insert_resource(DipCli::parse())
            //             .add_event::<Build>()
            //             .add_event::<Clean>()
            //             .set_runner(|app| Self::runner(app));
            //     }
            // }

            // impl CliPlugin {
            //     fn runner(mut app: dip::bevy::app::App) {
            //         let cli = app.world.get_resource::<DipCli>().unwrap();

            //         match cli.command {
            //             Commands::Build => {
            //                 app.world
            //                     .get_resource_mut::<dip::bevy::ecs::event::Events<Build>>()
            //                     .unwrap()
            //                     .send(Build);
            //             }
            //             Commands::Clean => {
            //                 app.world
            //                     .get_resource_mut::<dip::bevy::ecs::event::Events<Clean>>()
            //                     .unwrap()
            //                     .send(Clean);
            //             }
            //         }

            //         app.update();
            //     }
            // }
        };

        gen.into()
    }
}
