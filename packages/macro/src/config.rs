use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
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

        let mut base_path = quote! { "config/" };
        let mut prefix = quote! { "" };
        let mut separator = quote! { "__" };

        for a in &self.attrs {
            match a {
                NestedMeta::Meta(nm) => match nm {
                    Meta::NameValue(v) => match v.path.get_ident() {
                        Some(ident) => match ident.to_string().as_str() {
                            "path" => match &v.lit {
                                Lit::Str(ls) => {
                                    let value = ls.value();
                                    base_path = quote! { #value };
                                }
                                _ => {}
                            },
                            "prefix" => match &v.lit {
                                Lit::Str(ls) => {
                                    let value = ls.value();
                                    prefix = quote! { #value };
                                }
                                _ => {}
                            },
                            "separator" => match &v.lit {
                                Lit::Str(ls) => {
                                    let value = ls.value();
                                    separator = quote! { #value };
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

        let token = ConfigToken {
            config_struct: quote! { #config_struct },
            config_name: quote! { #ident },
            base_path,
            prefix,
            separator,
            ..Default::default()
        };

        token
    }
}

#[derive(Default)]
pub struct ConfigToken {
    config_struct: TokenStream2,
    config_name: TokenStream2,
    base_path: TokenStream2,
    prefix: TokenStream2,
    separator: TokenStream2,
}

impl ConfigToken {
    pub fn gen(&self) -> TokenStream {
        let Self {
            config_struct,
            config_name,
            base_path,
            prefix,
            separator,
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
                    let env_name = ::std::env::var("ENV").unwrap_or_else(|_| "local".into());
                    let base_path = #base_path;

                    let mut env = ::config::Environment::default().separator(#separator);

                    if #prefix != "" {
                        env = env.prefix(#prefix);
                    }

                    ::config::Config::builder()
                        .add_source(::config::File::with_name(&format!("{}/default", base_path)))
                        .add_source(
                            ::config::File::with_name(&format!("{}/{}", base_path, env_name))
                                .required(false),
                        )
                        .add_source(env)
                        .build()?
                        .try_deserialize()
                }
            }
        };

        gen.into()
    }
}
