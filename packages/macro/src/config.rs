use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::{AttributeArgs, ItemStruct, Lit, Meta, NestedMeta};

pub struct ConfigParser {
    attrs: AttributeArgs,
    config_struct: ItemStruct,
}

impl ConfigParser {
    pub fn new(attrs: AttributeArgs, config_struct: ItemStruct) -> Self {
        Self {
            attrs,
            config_struct,
        }
    }

    pub fn parse(&self) -> ConfigToken {
        let config_struct = &self.config_struct;
        let ident = &config_struct.ident;

        let mut default_file_path = quote! { "config/default.toml" };
        let mut default_file_format = quote! { Toml };
        let mut prefix = quote! { "" };
        let mut separator = quote! { "__" };
        let mut custom_paths = vec![];

        for a in &self.attrs {
            match a {
                NestedMeta::Meta(nm) => match nm {
                    Meta::NameValue(v) => match v.path.get_ident() {
                        Some(ident) => match ident.to_string().as_str() {
                            "default_file_path" => match &v.lit {
                                Lit::Str(ls) => {
                                    let value = ls.value();
                                    default_file_path = quote! { #value };
                                }
                                _ => {}
                            },
                            "default_file_format" => match &v.lit {
                                Lit::Str(ls) => {
                                    let value = ls.value().to_case(Case::UpperCamel);
                                    default_file_format = TokenStream2::from_str(&value).unwrap();
                                }
                                _ => {}
                            },
                            "env_prefix" => match &v.lit {
                                Lit::Str(ls) => {
                                    let value = ls.value();
                                    prefix = quote! { #value };
                                }
                                _ => {}
                            },
                            "env_separator" => match &v.lit {
                                Lit::Str(ls) => {
                                    let value = ls.value();
                                    separator = quote! { #value };
                                }
                                _ => {}
                            },
                            "override_user_config_path" => match &v.lit {
                                Lit::Str(ls) => {
                                    let value = ls.value();
                                    custom_paths.push(quote! {
                                        config_builder = config_builder
                                            .add_source(::config::File::with_name(#value).required(false));
                                    });
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        if custom_paths.is_empty() {
            custom_paths.push(
                quote! {
                    config_builder = config_builder
                        // $HOME/.config/{CARGO_PKG_NAME}
                        .add_source(
                            ::config::File::with_name(&format!(
                                "{home}/.config/{name}",
                                home = &home_dir_str,
                                name = PKG_NAME
                            ))
                            .required(false),
                        )
                        // $HOME/.config/{CARGO_PKG_NAME}/{CARGO_PKG_NAME}
                        .add_source(
                            ::config::File::with_name(&format!(
                                "{home}/.config/{name}/{name}",
                                home = &home_dir_str,
                                name = PKG_NAME
                            ))
                            .required(false),
                        )
                        // $HOME/.{CARGO_PKG_NAME}
                        .add_source(
                            ::config::File::with_name(&format!(
                                "{home}/.{name}",
                                home = &home_dir_str,
                                name = PKG_NAME
                            ))
                            .required(false),
                        )
                        // ./{CARGO_PKG_NAME}
                        .add_source(
                            ::config::File::with_name(&format!("{name}", name = PKG_NAME)).required(false),
                        );
                }
            );
        }

        let token = ConfigToken {
            config_struct: quote! { #config_struct },
            config_name: quote! { #ident },
            default_file_path,
            default_file_format,
            prefix,
            separator,
            custom_paths,
        };

        token
    }
}

pub struct ConfigToken {
    config_struct: TokenStream2,
    config_name: TokenStream2,
    default_file_path: TokenStream2,
    default_file_format: TokenStream2,
    prefix: TokenStream2,
    separator: TokenStream2,
    custom_paths: Vec<TokenStream2>,
}

impl ConfigToken {
    pub fn gen(&self) -> TokenStream {
        let Self {
            config_struct,
            config_name,
            default_file_path,
            default_file_format,
            prefix,
            separator,
            custom_paths,
        } = self;

        let gen = quote! {
                    pub struct ConfigPlugin;

                    impl ::bevy::app::Plugin for ConfigPlugin {
                        fn build(&self, app: &mut ::bevy::app::App) {
                            app.insert_resource(#config_name::new().unwrap());
                        }
                    }

                    #config_struct

                    impl #config_name {
                        pub fn new() -> Result<Self, ::config::ConfigError> {
                            const PKG_NAME: &str = env!("CARGO_PKG_NAME");

                            let home_dir = dirs::home_dir().unwrap();
                            let home_dir_str = home_dir.to_str().unwrap();

                            let mut config_builder = ::config::Config::builder()
                                // default config file in binary
                                .add_source(::config::File::from_str(
                                    include_str!(#default_file_path),
                                    ::config::FileFormat::#default_file_format,
                                ));

                            #(#custom_paths)*

                            config_builder = config_builder
                                .add_source(
                                    ::config::Environment::default()
                                        .prefix(#prefix)
                                        .separator(#separator)
                                );
        ;

                            match std::env::var("CONFIG") {
                                Ok(v) => {
                                    config_builder = config_builder.add_source(
                                        ::config::File::with_name(&format!(
                                            "examples/cli/config/config/{name}",
                                            name = v
                                        ))
                                    );
                                }
                                Err(_e) => {},
                            }

                            config_builder.build()?.try_deserialize()
                        }
                    }
                };

        gen.into()
    }
}
