use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::{collections::HashSet, str::FromStr};
use syn::{FnArg, ImplItem, ImplItemMethod, ItemImpl, ReturnType, Type};

pub struct UiActionParser {
    action_creator_impl: ItemImpl,
}

impl From<ItemImpl> for UiActionParser {
    fn from(action_creator_impl: ItemImpl) -> Self {
        Self {
            action_creator_impl,
        }
    }
}

impl UiActionParser {
    pub fn parse(&self) -> UiActionToken {
        let mut tokens = UiActionToken::default();
        tokens.action_creator_name = self.action_creator_name();
        tokens.action_creator_impl = self.action_creator_impl();

        let mut actions = HashSet::new();

        for item in self.action_creator_impl.clone().items {
            match item {
                ImplItem::Method(m) => {
                    let method_name_raw = &m.sig.ident;
                    let method_name = quote! { #method_name_raw };
                    let action = Self::action(&m);
                    let (arg_keys, args) = Self::method_args(&m);

                    actions.insert(action.to_string());
                    tokens
                        .methods
                        .push(self.method(&method_name, &action, args, arg_keys));
                }
                _ => {}
            }
        }

        for action_str in actions.iter() {
            let action = TokenStream2::from_str(&action_str).unwrap();
            let action_snake = TokenStream2::from_str(&action_str.to_case(Case::Snake)).unwrap();

            tokens.enum_variants.push(Self::enum_variant(&action));
            tokens.add_events.push(Self::add_event(&action));
            tokens
                .handler_args
                .push(Self::handler_arg(&action, &action_snake));
            tokens.handlers.push(Self::handler(&action, &action_snake));
        }

        tokens
    }

    fn action_creator_name(&self) -> TokenStream2 {
        let name = match &**&self.action_creator_impl.self_ty {
            Type::Path(p) => {
                let name = &p.path.segments[0].ident;
                quote! { #name }
            }
            _ => {
                panic!("Make sure UiState struct has right structure");
            }
        };
        quote! { #name }
    }

    fn action_creator_impl(&self) -> TokenStream2 {
        let input = &self.action_creator_impl;
        quote! { #input }
    }

    // example
    // pub fn create_todo(title: &String) -> Self {
    //     Self::CreateTodo(ActionCreator::create_todo(title))
    // }
    fn method(
        &self,
        method_name: &TokenStream2,
        action: &TokenStream2,
        args: Vec<TokenStream2>,
        arg_keys: Vec<TokenStream2>,
    ) -> TokenStream2 {
        let action_creator_name = self.action_creator_name();

        quote! {
            pub fn #method_name(#(#args)*) -> Self {
                Self::#action(#action_creator_name::#method_name(#(#arg_keys)*))
            }
        }
    }

    fn method_args(method: &ImplItemMethod) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
        let mut arg_keys = vec![];
        let mut args = vec![];
        for arg in method.sig.inputs.iter() {
            match arg {
                FnArg::Typed(pt) => {
                    let ident = &pt.pat;
                    arg_keys.push(quote! { #ident, });
                }
                _ => {}
            }
            args.push(quote! { #arg, });
        }

        (arg_keys, args)
    }

    fn action(method: &ImplItemMethod) -> TokenStream2 {
        match &method.sig.output {
            ReturnType::Type(_, return_type) => match *return_type.clone() {
                Type::Path(type_path) => {
                    let action = type_path.path.segments[0].ident.clone();
                    quote! { #action }
                }
                _ => {
                    panic!("Cannot find event name. Make sure to sepcify return event in action creator methods.");
                }
            },
            _ => {
                panic!("Cannot find event name. Make sure to sepcify return event in action creator methods.");
            }
        }
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
pub struct UiActionToken {
    enum_variants: Vec<TokenStream2>,
    add_events: Vec<TokenStream2>,
    handler_args: Vec<TokenStream2>,
    handlers: Vec<TokenStream2>,
    action_creator_name: TokenStream2,
    action_creator_impl: TokenStream2,
    methods: Vec<TokenStream2>,
}

impl UiActionToken {
    pub fn gen(&self) -> TokenStream {
        let Self {
            enum_variants,
            add_events,
            handler_args,
            handlers,
            action_creator_name,
            action_creator_impl,
            methods,
        } = self;

        let gen = quote! {
            pub struct UiActionPlugin;

            impl Plugin for UiActionPlugin {
                fn build(&self, app: &mut App) {
                    app
                        #(#add_events)*
                        .add_system_to_stage(UiStage::Action, send_ui_action_event);
                }
            }

            #[derive(Clone, Debug)]
            pub enum UiAction {
                #(#enum_variants)*
            }

            struct #action_creator_name;

            #action_creator_impl

            impl UiAction {
                #(#methods)*
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
        };

        gen.into()
    }
}
