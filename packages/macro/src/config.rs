use proc_macro::TokenStream;
// use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
// use std::str::FromStr;
use syn::ItemStruct;

pub struct ConfigParser {
    config_struct: ItemStruct,
}

impl ConfigParser {
    pub fn new(config_struct: ItemStruct) -> Self {
        Self { config_struct }
    }

    pub fn parse(&self) -> ConfigToken {
        let ident = &self.config_struct.ident;

        let mut token = ConfigToken {
            // config_name: quote! { #ident },
            ..Default::default()
        };

        token
    }
}

#[derive(Default)]
pub struct ConfigToken {}

impl ConfigToken {
    pub fn gen(&self) -> TokenStream {
        let Self {} = self;
        let gen = quote! {
            pub struct ConfigPlugin;

            impl ::bevy::app::Plugin for ConfigPlugin {
                fn build(&self, app: &mut ::bevy::app::App) {
                    app.insert_resource(Config::new().unwrap());
                }
            }

            impl Config {
                pub fn new() -> Result<Self, ::config::ConfigError> {
                    let run_mode = ::std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
                    let base_path = "examples/cli/config/config";

                    ::config::Config::builder()
                        .add_source(::config::File::with_name(&format!("{}/default", base_path)))
                        .add_source(
                            ::config::File::with_name(&format!("{}/config/{}", base_path, run_mode))
                                .required(false),
                        )
                        .add_source(::config::File::with_name(&format!("{}/local", base_path)))
                        .add_source(::config::Environment::with_prefix("APP").separator("__"))
                        .build()?
                        .try_deserialize()
                }
            }
        };

        gen.into()
    }
}
