use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::{collections::HashSet, fmt, str::FromStr};
use syn::{FnArg, ImplItem, ImplItemMethod, ItemImpl, ReturnType, Type};

pub struct ActionParser {
    action_type: ActionType,
    action_creator_impl: ItemImpl,
}

impl ActionParser {
    pub fn async_action(action_creator_impl: ItemImpl) -> Self {
        Self {
            action_type: ActionType::AsyncAction,
            action_creator_impl,
        }
    }

    pub fn ui_action(action_creator_impl: ItemImpl) -> Self {
        Self {
            action_type: ActionType::UiAction,
            action_creator_impl,
        }
    }

    pub fn parse(&self) -> ActionToken {
        let mut tokens = ActionToken {
            plugin_name: self.plugin_name(),
            action_name: self.action_name(),
            action_creator_impl: self.action_creator_impl(),
            action_creator_name: self.action_creator_name(),
            handler_name: self.handler_name(),
            ..Default::default()
        };

        let mut actions = HashSet::new();

        for item in self.action_creator_impl.clone().items {
            match item {
                ImplItem::Method(m) => {
                    let method_name_raw = &m.sig.ident;
                    let method_name = quote! { #method_name_raw };
                    let action = Self::action(&m);
                    let (arg_keys, args) = Self::method_args(&m);
                    let (async_key, await_key) = if m.sig.asyncness.is_some() {
                        (quote! { async }, quote! { .await })
                    } else {
                        (quote! {}, quote! {})
                    };

                    actions.insert(action.to_string());
                    tokens.action_methods.push(self.action_method(
                        &method_name,
                        &action,
                        args,
                        arg_keys,
                        &async_key,
                        &await_key,
                    ));
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
            tokens.handlers.push(self.handler(&action, &action_snake));
        }

        tokens
    }

    fn plugin_name(&self) -> TokenStream2 {
        TokenStream2::from_str(self.action_type.plugin_name()).unwrap()
    }

    fn action_name(&self) -> TokenStream2 {
        TokenStream2::from_str(self.action_type.name()).unwrap()
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

    fn handler_name(&self) -> TokenStream2 {
        TokenStream2::from_str(self.action_type.handler_name()).unwrap()
    }

    // example
    // pub fn create_todo(title: &String) -> Self {
    //     Self::CreateTodo(ActionCreator::create_todo(title))
    // }
    fn action_method(
        &self,
        method_name: &TokenStream2,
        action: &TokenStream2,
        args: Vec<TokenStream2>,
        arg_keys: Vec<TokenStream2>,
        async_key: &TokenStream2,
        await_key: &TokenStream2,
    ) -> TokenStream2 {
        let action_creator_name = self.action_creator_name();

        quote! {
            pub #async_key fn #method_name(#(#args)*) -> Self {
                Self::#action(#action_creator_name::#method_name(#(#arg_keys)*)#await_key)
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
    fn handler(&self, action: &TokenStream2, action_snake: &TokenStream2) -> TokenStream2 {
        let action_type = TokenStream2::from_str(&self.action_type.to_string()).unwrap();

        quote! {
            #action_type::#action(event) => {
                #action_snake.send(event.clone());
            }
        }
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
}

#[derive(Default)]
pub struct ActionToken {
    plugin_name: TokenStream2,
    action_name: TokenStream2,
    action_creator_impl: TokenStream2,
    action_creator_name: TokenStream2,
    enum_variants: Vec<TokenStream2>,
    add_events: Vec<TokenStream2>,
    action_methods: Vec<TokenStream2>,
    handler_name: TokenStream2,
    handler_args: Vec<TokenStream2>,
    handlers: Vec<TokenStream2>,
}

impl ActionToken {
    pub fn gen(&self) -> TokenStream {
        let Self {
            plugin_name,
            action_name,
            action_creator_name,
            action_creator_impl,
            enum_variants,
            add_events,
            action_methods,
            handler_name,
            handler_args,
            handlers,
        } = self;

        let gen = quote! {
            pub struct #plugin_name;

            impl ::dip::bevy::app::Plugin for #plugin_name {
                fn build(&self, app: &mut App) {
                    app
                        .add_event::<#action_name>()
                        #(#add_events)*
                        .add_system_to_stage(::dip::core::schedule::DipStage::Action, #handler_name);
                }
            }

            #[derive(Clone, Debug)]
            pub enum #action_name {
                #(#enum_variants)*
            }

            struct #action_creator_name;

            #action_creator_impl

            impl #action_name {
                #(#action_methods)*
            }

            pub fn #handler_name(
                mut events: EventReader<#action_name>,
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

pub enum ActionType {
    AsyncAction,
    UiAction,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionType::AsyncAction => write!(f, "AsyncAction"),
            ActionType::UiAction => write!(f, "UiAction"),
        }
    }
}

impl ActionType {
    fn name(&self) -> &'static str {
        match self {
            ActionType::AsyncAction => "AsyncAction",
            ActionType::UiAction => "UiAction",
        }
    }

    fn plugin_name(&self) -> &'static str {
        match self {
            ActionType::AsyncAction => "AsyncActionPlugin",
            ActionType::UiAction => "UiActionPlugin",
        }
    }

    fn handler_name(&self) -> &'static str {
        match self {
            ActionType::AsyncAction => "send_async_action_event",
            ActionType::UiAction => "send_ui_action_event",
        }
    }
}
