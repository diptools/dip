use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
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

        let token = ConfigToken {
            config_name: quote! { #ident },
        };

        token
    }
}

pub struct ConfigToken {
    config_name: TokenStream2,
}

impl ConfigToken {
    pub fn gen(&self) -> TokenStream {
        let Self { config_name } = self;

        let gen = quote! {
            pub struct ConfigPlugin {
                default_paths: bool,
                env_prefix: Option<&'static str>,
                env_separator: &'static str,
                default_file_str: &'static str,
                default_file_format: ::config::FileFormat,
            }

            impl ::dip::bevy::app::Plugin for ConfigPlugin {
                fn build(&self, app: &mut ::bevy::app::App) {
                    use ::bevy::ecs::system::IntoSystem;

                    app.insert_resource(#config_name::builder(&self))
                        .add_startup_system(build_config);
                }
            }

            impl Default for ConfigPlugin {
                fn default() -> Self {
                    Self {
                        default_paths: true,
                        env_prefix: None,
                        env_separator: "__",
                        default_file_str: include_str!("config/default.toml"),
                        default_file_format: ::config::FileFormat::Toml,
                    }
                }
            }

            impl ConfigPlugin {
                pub fn new() -> Self {
                    Self::default()
                }

                pub fn default_paths(mut self, default_paths: bool) -> Self {
                    self.default_paths = default_paths;
                    self
                }

                pub fn env_prefix(mut self, prefix: &'static str) -> Self {
                    self.env_prefix = Some(prefix);
                    self
                }

                pub fn env_separator(mut self, separator: &'static str) -> Self {
                    self.env_separator = separator;
                    self
                }

                pub fn default_from_str(mut self, default_str: &'static str) -> Self {
                    self.default_file_str = default_str;
                    self
                }
            }

            impl #config_name {
                pub fn builder(plugin: &ConfigPlugin) -> ::config::builder::ConfigBuilder<::config::builder::DefaultState> {
                    const PKG_NAME: &str = env!("CARGO_PKG_NAME");

                    let home_dir = dirs::home_dir().unwrap();
                    let home_dir_str = home_dir.to_str().unwrap();

                    let mut builder = ::config::Config::builder();
                    let mut env = ::config::Environment::default().separator(plugin.env_separator);

                    if let Some(prefix) = &plugin.env_prefix {
                        env = env.prefix(&prefix);
                    }

                    if plugin.default_paths {
                        builder = builder
                            // default config file in binary
                            .add_source(::config::File::from_str(
                                plugin.default_file_str,
                                plugin.default_file_format,
                            ))
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
                            )
                            .add_source(env);
                    }


                    if let Ok(name) = std::env::var("CONFIG_PATH") {
                        builder = builder.add_source(
                            ::config::File::with_name(&name)
                        );
                    }

                    builder
                }
            }

            pub fn build_config(
                builder: ::bevy::ecs::system::Res<::config::builder::ConfigBuilder<::config::builder::DefaultState>>,
                mut config: Option<::bevy::ecs::system::ResMut<#config_name>>,
                mut commands: Commands,
            ) {
                let c = builder
                    .clone()
                    .build()
                    .unwrap()
                    .try_deserialize::<'static, #config_name>()
                    .unwrap();

                if config.is_none() {
                    commands.insert_resource(c);
                }
            }
        };

        gen.into()
    }
}
