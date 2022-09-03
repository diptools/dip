use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::{ItemStruct, Type};

pub struct UiStateParser {
    input: ItemStruct,
}

impl From<ItemStruct> for UiStateParser {
    fn from(input: ItemStruct) -> Self {
        Self { input }
    }
}

impl UiStateParser {
    pub fn parse(&self) -> UiStateTokenStreams {
        let mut tokens = UiStateTokenStreams::default();

        for f in &self.input.fields {
            let name_str = f.ident.as_ref().unwrap().to_string();
            let type_raw = &f.ty;
            let r#type = quote! { #type_raw };
            let name_upper_camel =
                TokenStream2::from_str(&name_str.to_case(Case::UpperCamel)).unwrap();
            let name_upper_snake =
                TokenStream2::from_str(&name_str.to_case(Case::UpperSnake)).unwrap();
            let type_name = match &f.ty {
                Type::Path(p) => {
                    let name = &p.path.segments[0].ident;
                    quote! { #name }
                }
                _ => {
                    panic!("Make sure UiState struct has right structure");
                }
            };

            tokens
                .atom_quotes
                .push(Self::atom_quote(&name_upper_snake, &r#type, &type_name));
            tokens
                .enum_variants
                .push(Self::enum_variant(&name_upper_camel, &r#type));
            tokens
                .variant_handlers
                .push(Self::variant_handler(&name_upper_snake, &name_upper_camel))
        }

        tokens
    }
    // example: pub static TODO_LIST: Atom<Vec<UiTodo>> = |_| Vec::default();
    fn atom_quote(
        name_upper_snake: &TokenStream2,
        r#type: &TokenStream2,
        type_name: &TokenStream2,
    ) -> TokenStream2 {
        quote! {
            pub static #name_upper_snake: Atom<#r#type> = |_| #type_name::default();
        }
    }

    // example: TodoList(Vec<UiTodo>),
    fn enum_variant(name_upper_camel: &TokenStream2, r#type: &TokenStream2) -> TokenStream2 {
        quote! {
            #name_upper_camel(#r#type),
        }
    }

    // example: UiState::TodoList(x) => root.set(TODO_LIST.unique_id(), x),
    fn variant_handler(
        name_upper_snake: &TokenStream2,
        name_upper_camel: &TokenStream2,
    ) -> TokenStream2 {
        quote! {
            UiState::#name_upper_camel(x) => root.set(#name_upper_snake.unique_id(), x),
        }
    }
}

#[derive(Default)]
pub struct UiStateTokenStreams {
    atom_quotes: Vec<TokenStream2>,
    enum_variants: Vec<TokenStream2>,
    variant_handlers: Vec<TokenStream2>,
}

impl UiStateTokenStreams {
    pub fn gen(&self) -> TokenStream {
        let Self {
            atom_quotes,
            enum_variants,
            variant_handlers,
        } = self;

        let gen = quote! {
            use bevy_dioxus::{
                bevy::{
                    ecs::system::Res,
                    log::{error, trace},
                },
                desktop::futures_intrusive::channel::{shared::Sender, TrySendError},
                dioxus::fermi::{Atom, AtomRoot, Readable},
            };
            use std::rc::Rc;

            #(#atom_quotes)*

            #[derive(Clone, Debug)]
            pub enum UiState {
                #(#enum_variants)*
            }

            impl UiStateHandler for UiState {
                fn handler(self, root: Rc<AtomRoot>) {
                    match self {
                        #(#variant_handlers)*
                    }
                }
            }

            pub struct UiStatePlugin;

            impl Plugin for UiStatePlugin {
                fn build(&self, app: &mut App) {
                    app.add_event::<UiState>()
                        .add_system_to_stage(UiStage::Render, apply_ui_state);
                }
            }

            fn apply_ui_state(
                mut events: EventReader<UiState>,
                ui_state_tx: Res<Sender<UiState>>,
            ) {
                for e in events.iter() {
                    trace!("apply_ui_state");

                    match ui_state_tx.try_send(e.clone()) {
                        Ok(()) => {}
                        Err(e) => match e {
                            TrySendError::Full(e) => {
                                error!(
                                    "Failed to send UiState: channel is full: event: {:?}",
                                    e
                                );
                            }
                            TrySendError::Closed(e) => {
                                error!(
                                    "Failed to send UiState: channel is closed: event: {:?}",
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
}
