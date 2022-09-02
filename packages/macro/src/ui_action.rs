use convert_case::{Case, Casing};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::{Ident, Type};

pub struct UiActionParser {
    actions: Vec<Ident>,
}

impl From<Vec<Type>> for UiActionParser {
    fn from(args: Vec<Type>) -> Self {
        let mut actions = vec![];
        for arg in args.into_iter() {
            match arg {
                Type::Path(type_path) => {
                    let action = &type_path.path.segments.first().unwrap().ident;
                    actions.push(action.clone());
                }
                _ => {}
            }
        }

        Self { actions }
    }
}

impl UiActionParser {
    pub fn parse(&self) -> UiActionTokenStreams {
        let mut tokens = UiActionTokenStreams::default();

        for action in self.actions.iter() {
            let action = quote! { #action };
            let action_snake =
                TokenStream2::from_str(&action.to_string().to_case(Case::Snake)).unwrap();
            tokens.enum_variants.push(Self::enum_variant(&action));
            tokens.add_events.push(Self::add_event(&action));
            tokens
                .handler_args
                .push(Self::handler_arg(&action, &action_snake));
            tokens.handlers.push(Self::handler(&action, &action_snake));
        }
        tokens
    }

    // example: CreateTodo(CreateTodo),
    fn enum_variant(action: &TokenStream2) -> TokenStream2 {
        quote! {
            #action(#r#action),
        }
    }

    // example: .add_event::<CreateTodo>()
    fn add_event(action: &TokenStream2) -> TokenStream2 {
        quote! {
            .add_event::<#action>()
        }
    }

    // example: mut create_todo: EventWriter<CreateTodo>,
    fn handler_arg(action: &TokenStream2, action_snake: &TokenStream2) -> TokenStream2 {
        quote! {
            mut #action_snake: EventWriter<#action>,
        }
    }

    // example:
    // UiAction::CreateTodo(event) => {
    //     create_todo.send(event.clone());
    // }
    fn handler(action: &TokenStream2, action_snake: &TokenStream2) -> TokenStream2 {
        quote! {
            UiAction::#action(event) => {
                #action_snake.send(event.clone());
            }
        }
    }
}

#[derive(Default)]
pub struct UiActionTokenStreams {
    pub enum_variants: Vec<TokenStream2>,
    pub add_events: Vec<TokenStream2>,
    pub handler_args: Vec<TokenStream2>,
    pub handlers: Vec<TokenStream2>,
}

impl UiActionTokenStreams {
    pub fn gen(&self) -> TokenStream2 {
        let Self {
            enum_variants,
            add_events,
            handler_args,
            handlers,
        } = self;

        let gen = quote! {
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
                    app
                        #(#add_events)*
                        .add_system_to_stage(UiStage::Action, send_ui_action_event);
                }
            }
        };

        gen.into()
    }
}
