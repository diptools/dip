extern crate proc_macro;

mod global_state;
mod ui_action;

use global_state::{GlobalStateParser, GlobalStateTokens};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use ui_action::{UiActionParser, UiActionTokens};

#[proc_macro_attribute]
pub fn global_state(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let GlobalStateTokens {
        atom_quotes,
        enum_variants,
        variant_handlers,
    } = GlobalStateParser::from(input).parse();

    let gen = quote! {
        use bevy_dioxus::{
            bevy::{
                app::Plugin,
                ecs::system::Res,
                log::{error, trace},
            },
            core::schedule::UiStage,
            desktop::futures_intrusive::channel::{shared::Sender, TrySendError},
            dioxus::fermi::{Atom, AtomRoot, Readable},
        };
        use std::rc::Rc;

        #(#atom_quotes)*

        #[derive(Clone, Debug)]
        pub enum GlobalState {
            #(#enum_variants)*
        }

        impl GlobalStateHandler for GlobalState {
            fn handler(self, root: Rc<AtomRoot>) {
                match self {
                    #(#variant_handlers)*
                }
            }
        }

        pub struct GlobalStatePlugin;

        impl Plugin for GlobalStatePlugin {
            fn build(&self, app: &mut App) {
                app.add_event::<GlobalState>()
                    .add_system_to_stage(UiStage::Render, apply_global_state_command);
            }
        }

        fn apply_global_state_command(
            mut events: EventReader<GlobalState>,
            global_state_tx: Res<Sender<GlobalState>>,
        ) {
            for e in events.iter() {
                trace!("apply_global_state_command");
                match global_state_tx.try_send(e.clone()) {
                    Ok(()) => {}
                    Err(e) => match e {
                        TrySendError::Full(e) => {
                            error!(
                                "Failed to send VDomCommand: channel is full: event: {:?}",
                                e
                            );
                        }
                        TrySendError::Closed(e) => {
                            error!(
                                "Failed to send VDomCommand: channel is closed: event: {:?}",
                                e
                            );
                        }
                    },
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn ui_action(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let UiActionTokens {
        enum_variants,
        handler_args,
        handlers,
    } = UiActionParser::from(input).parse();

    let gen = quote! {
        use bevy::app::{App, Plugin};
        use bevy_dioxus::core::schedule::UiStage;

        #[derive(Clone, Debug)]
        pub enum UiAction {
            #(#enum_variants)*
        }

        pub fn send_ui_action_event(
            mut events: EventReader<UiAction>,
            #(#handler_args)*
        ) {
            for action in events.iter() {
                match action {
                    #(#handlers)*
                }
            }
        }

        pub struct UiActionPlugin;

        impl Plugin for UiActionPlugin {
            fn build(&self, app: &mut App) {
                app.add_system_to_stage(UiStage::Action, send_ui_action_event);
            }
        }
    };

    gen.into()
}
