#[macro_use]
extern crate lazy_static;
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::sync::Mutex;
use syn::{parse_macro_input, DeriveInput};

lazy_static! {
    static ref DEPS: Mutex<Vec<String>> = Mutex::new(Default::default());
}

#[proc_macro_derive(GlobalState)]
pub fn global_atom(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    DEPS.lock().as_mut().unwrap().push(input.ident.to_string());

    let name = &input.ident;
    let atom_name = format_ident!("{}", &name.to_string().to_uppercase());

    let gen = quote! {
        static #atom_name: Atom<#name> = |_| #name::default();
    };

    gen.into()
}

#[proc_macro_derive(GlobalStatePlugin)]
pub fn derive_global_state_plugin(_input: TokenStream) -> TokenStream {
    let mut enum_inners = vec![];
    let mut handler_inners = vec![];
    let mut systems = vec![];
    let mut add_systems = vec![];

    for name in DEPS.lock().unwrap().iter() {
        let atom_name = format_ident!("{}", name.to_uppercase());
        let system_name = format_ident!("apply_{}", name.to_lowercase());
        let name = format_ident!("{}", name);

        let enum_inner = quote! { #name(#name) };
        let handler_inner = quote! { GlobalState::#name(x) => root.set(#atom_name.unique_id(), x) };
        let system = quote! {
            fn #system_name(
                query: Query<&#name, Changed<#name>>,
                vdom_tx: Res<Sender<VDomCommand<GlobalState>>>,
            ) {
                for x in query.iter() {
                    match vdom_tx.try_send(VDomCommand::GlobalState(GlobalState::#name(x.clone()))) {
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
        let add_system = quote! {
            app.add_system(#system_name);
        };

        enum_inners.push(enum_inner);
        handler_inners.push(handler_inner);
        systems.push(system);
        add_systems.push(add_system);
    }

    let gen = quote! {
        use dioxus::fermi::{AtomRoot, Readable};
        use std::rc::Rc;

        #[derive(Debug)]
        enum GlobalState {
            #(#enum_inners),*
        }

        impl GlobalStateHandler for GlobalState {
            fn handler(self, root: Rc<AtomRoot>) {
                match self {
                    #(#handler_inners),*
                }
            }
        }

        impl Plugin for GlobalStatePlugin {
            fn build(&self, app: &mut App) {
                #(#add_systems)*;
            }
        }

        #(#systems)*;

    };

    gen.into()
}

#[proc_macro_derive(GlobalStateCommand)]
pub fn derive_global_state_command(input: TokenStream) -> TokenStream {
    let gen = quote! {
        use bevy_dioxus::desktop::event::VDomCommand;
        use dioxus::fermi::{Atom, AtomRoot, Readable};
        use futures_intrusive::channel::{shared::Sender, TrySendError};
        use std::rc::Rc;

        pub static TODO_LIST: Atom<Vec<UiTodo>> = |_| vec![];


        impl GlobalStateHandler for GlobalStateCommand {
            fn handler(self, root: Rc<AtomRoot>) {
                match self {
                    GlobalStateCommand::TodoList(x) => root.set(TODO_LIST.unique_id(), x),
                }
            }
        }

        pub struct GlobalStatePlugin;


        impl Plugin for GlobalStatePlugin {
            fn build(&self, app: &mut App) {
                app.add_event::<GlobalStateCommand>()
                    .add_system(apply_global_state_command);
                // #(#add_systems)*;
            }
        }

        // #(#systems)*;
        fn apply_global_state_command(
            mut events: EventReader<GlobalStateCommand>,
            vdom_tx: Res<Sender<VDomCommand<GlobalStateCommand>>>,
        ) {
            for e in events.iter() {
                match vdom_tx.try_send(VDomCommand::GlobalState(e.clone())) {
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
