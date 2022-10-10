use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use std::str::FromStr;
use syn::ItemStruct;

pub struct CliParser {
    cli_struct: ItemStruct,
}

impl CliParser {
    pub fn new(cli_struct: ItemStruct) -> Self {
        Self { cli_struct }
    }

    pub fn parse(&self) -> CliToken {
        let ident = &self.cli_struct.ident;

        let mut token = CliToken {
            cli_name: quote! { #ident },
            ..Default::default()
        };

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
                                            let ty = &f.ty;
                                            token.add_event = quote! { .add_event::<#ty>() };
                                            token.insert_subcommand_resource = quote! {
                                                .insert_resource(cli.#subcommand_name.clone())
                                            };
                                            let subcommand_handler_name = TokenStream2::from_str(
                                                &format!("handle_{}", subcommand_name),
                                            )
                                            .unwrap();
                                            token.add_subcommand_handler = quote! {
                                                .add_system_to_stage(
                                                    ::dip::core::schedule::UiStage::Action,
                                                    convert_subcommand_to_event.before(
                                                        #subcommand_handler_name
                                                    )
                                                )
                                            };
                                            token.subcommand_handler = quote! {
                                                fn convert_subcommand_to_event(
                                                    subcommand: ::dip::bevy::ecs::system::Res<#ty>,
                                                    mut #subcommand_name: ::dip::bevy::ecs::event::EventWriter<#ty>,
                                                ) {
                                                    #subcommand_name.send(subcommand.clone());
                                                }
                                            };
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

        token
    }
}

#[derive(Default)]
pub struct CliToken {
    cli_name: TokenStream2,
    insert_subcommand_resource: TokenStream2,
    add_event: TokenStream2,
    add_subcommand_handler: TokenStream2,
    subcommand_handler: TokenStream2,
}

impl CliToken {
    pub fn gen(&self) -> TokenStream {
        let Self {
            cli_name,
            insert_subcommand_resource,
            add_event,
            add_subcommand_handler,
            subcommand_handler,
        } = self;

        let gen = quote! {
            pub struct CliPlugin<AsyncAction> {
                async_action_type: std::marker::PhantomData<AsyncAction>,
                repl: bool,
            }

            impl<AsyncAction> CliPlugin<AsyncAction> {
                pub fn oneshot() -> Self {
                    Self {
                        async_action_type: std::marker::PhantomData,
                        repl: false,
                    }
                }

                pub fn repl() -> Self {
                    Self {
                        async_action_type: std::marker::PhantomData,
                        repl: true,
                    }
                }
            }

            impl<AsyncAction> ::dip::bevy::app::Plugin for CliPlugin<AsyncAction>
            where
                AsyncAction: 'static + Send + Sync + Clone,
            {
                fn build(&self, app: &mut ::bevy::app::App) {
                    use ::clap::Parser;
                    use ::dip::bevy::ecs::{
                        schedule::ParallelSystemDescriptorCoercion,
                        system::IntoSystem,
                    };

                    let cli = #cli_name::parse();
                    let repl = self.repl;

                    app.add_plugin(::dip::core::schedule::UiSchedulePlugin)
                        #insert_subcommand_resource
                        .insert_resource(cli)
                        #add_event
                        .set_runner(move |mut app| {
                            if !repl {
                                app.update();
                            } else {
                                let (async_action_tx, mut async_action_rx) = ::tokio::sync::mpsc::channel::<AsyncAction>(8);
                                let async_action = ::dip::core::task::AsyncActionPool::new(async_action_tx.clone());
                                app.world.insert_resource(async_action);
                                app.update();

                                loop {
                                    if let Some(app_exit_events) = app.world.get_resource::<::dip::bevy::ecs::event::Events<::dip::bevy::app::AppExit>>() {
                                        let mut app_exit_event_reader = ::dip::bevy::ecs::event::ManualEventReader::<::dip::bevy::app::AppExit>::default();
                                        if app_exit_event_reader.iter(app_exit_events).last().is_some() {
                                            break
                                        }
                                    }

                                    while let Ok(action) = async_action_rx.try_recv() {
                                        let mut events = app
                                            .world
                                            .get_resource_mut::<::dip::bevy::ecs::event::Events<AsyncAction>>()
                                            .expect("Provide AsyncAction event to bevy");
                                        events.send(action);

                                        app.update();
                                    }
                                }
                            };
                        })
                        #add_subcommand_handler;
                }
            }

            #subcommand_handler
        };

        gen.into()
    }
}
