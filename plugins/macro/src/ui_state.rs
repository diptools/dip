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
    pub fn parse(&self) -> UiStateToken {
        let mut tokens = UiStateToken {
            ui_state_name: self.ui_state_name(),
            ..Default::default()
        };

        for f in &self.input.fields {
            let name_str = f.ident.as_ref().unwrap().to_string();
            let ident = f.ident.as_ref().unwrap();
            let name = quote! { #ident };
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
            let system_name =
                TokenStream2::from_str(format!("dispatch_{}", &name).as_str()).unwrap();

            tokens
                .atom_quotes
                .push(Self::atom_quote(&name_upper_snake, &r#type, &type_name));
            tokens
                .enum_variants
                .push(Self::enum_variant(&name_upper_camel, &r#type));
            tokens
                .variant_handlers
                .push(self.variant_handler(&name_upper_snake, &name_upper_camel));
            tokens.init_resources.push(Self::init_resource(&r#type));
            tokens
                .add_dispatch_systems
                .push(Self::add_dispatch_system(&system_name));
            tokens.dispatch_systems.push(self.dispatch_system(
                &system_name,
                &r#type,
                &name,
                &name_upper_camel,
            ));
        }

        tokens
    }

    fn ui_state_name(&self) -> TokenStream2 {
        let ui_state_name = &self.input.ident;
        quote! { #ui_state_name }
    }

    // example: pub static TODO_LIST: ::dip::dioxus::fermi::Atom<Vec<UiTodo>> = |_| Vec::default();
    fn atom_quote(
        name_upper_snake: &TokenStream2,
        r#type: &TokenStream2,
        type_name: &TokenStream2,
    ) -> TokenStream2 {
        quote! {
            pub static #name_upper_snake: ::dip::dioxus::fermi::Atom<#r#type> = |_| #type_name::default();
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
        &self,
        name_upper_snake: &TokenStream2,
        name_upper_camel: &TokenStream2,
    ) -> TokenStream2 {
        let ui_state_name = self.ui_state_name();

        quote! {
            #ui_state_name::#name_upper_camel(x) => root.set(#name_upper_snake.unique_id(), x),
        }
    }

    // example: .init_resource::<Vec<UiTodo>>()
    fn init_resource(r#type: &TokenStream2) -> TokenStream2 {
        quote! {
            .init_resource::<#r#type>()
        }
    }

    // example: .add_system_to_stage(DipStage::Apply, dispatch_todo_list)
    fn add_dispatch_system(system_name: &TokenStream2) -> TokenStream2 {
        quote! {
            .add_system_to_stage(::dip::core::schedule::DipStage::Apply, #system_name)
        }
    }

    // example:
    // fn dispatch_todo_list(
    //     todo_list: Res<Vec<UiTodo>>,
    //     mut ui_state_tx: Res<Sender<UiState>>,
    // ) {
    //     if todo_list.is_changed() {
    //         trace!("dispatch_todo_list");
    //         match ui_state_tx.try_send(UiState::todoList(todo_list.clone())) {
    //             Ok(()) => {}
    //             Err(e) => match e {
    //                 TrySendError::Full(e) => {
    //                     error!("Failed to send UiState: channel is full: event: {:?}", e);
    //                 }
    //                 TrySendError::Closed(e) => {
    //                     error!("Failed to send UiState: channel is closed: event: {:?}", e);
    //                 }
    //             },
    //         }
    //     }
    // }
    fn dispatch_system(
        &self,
        system_name: &TokenStream2,
        r#type: &TokenStream2,
        name: &TokenStream2,
        name_upper_camel: &TokenStream2,
    ) -> TokenStream2 {
        let ui_state_name = self.ui_state_name();
        let system_name_str = system_name.to_string();

        quote! {
            fn #system_name(
                #name: ::dip::bevy::ecs::system::Res<#r#type>,
                ui_state_tx: ::dip::bevy::ecs::system::Res<tokio::sync::mpsc::Sender<#ui_state_name>>,
            ) {
                if #name.is_changed() {
                    trace!(#system_name_str);
                    match ui_state_tx.try_send(#ui_state_name::#name_upper_camel(#name.clone())) {
                        Ok(()) => {}
                        Err(e) => match e {
                            ::tokio::sync::mpsc::error::TrySendError::Full(e) => {
                                ::dip::bevy::log::error!(
                                    "Failed to send UiState: channel is full: event: {:?}",
                                    e
                                );
                            }
                            ::tokio::sync::mpsc::error::TrySendError::Closed(e) => {
                                ::dip::bevy::log::error!(
                                    "Failed to send UiState: channel is closed: event: {:?}",
                                    e
                                );
                            }
                        },
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct UiStateToken {
    ui_state_name: TokenStream2,
    atom_quotes: Vec<TokenStream2>,
    enum_variants: Vec<TokenStream2>,
    variant_handlers: Vec<TokenStream2>,
    init_resources: Vec<TokenStream2>,
    add_dispatch_systems: Vec<TokenStream2>,
    dispatch_systems: Vec<TokenStream2>,
}

impl UiStateToken {
    pub fn gen(&self) -> TokenStream {
        let Self {
            ui_state_name,
            atom_quotes,
            enum_variants,
            variant_handlers,
            init_resources,
            add_dispatch_systems,
            dispatch_systems,
        } = self;

        let gen = quote! {
            #(#atom_quotes)*

            #[derive(Clone, Debug)]
            pub enum #ui_state_name {
                #(#enum_variants)*
            }

            impl UiStateHandler for #ui_state_name {
                fn handler(self, root: std::rc::Rc<::dip::dioxus::fermi::AtomRoot>) {
                    use ::dip::dioxus::fermi::Readable;

                    match self {
                        #(#variant_handlers)*
                    }
                }
            }

            pub struct UiStatePlugin;

            impl ::dip::bevy::app::Plugin for UiStatePlugin {
                fn build(&self, app: &mut App) {
                    use ::dip::bevy::ecs::{
                        schedule::ParallelSystemDescriptorCoercion,
                        system::IntoSystem,
                    };

                    app #(#init_resources)*
                        #(#add_dispatch_systems)*;
                }
            }

            #(#dispatch_systems)*
        };

        gen.into()
    }
}
