use convert_case::{Case, Casing};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::{Data, DeriveInput, Field, Ident, Type};

pub struct UiActionParser {
    fields: Vec<UiActionField>,
}

impl From<DeriveInput> for UiActionParser {
    fn from(input: DeriveInput) -> Self {
        match input.data {
            Data::Struct(data) => {
                let mut fields = vec![];
                for f in data.fields {
                    fields.push(UiActionField::from(f));
                }
                Self { fields }
            }
            _ => {
                panic!("ui_action macro can only be used for struct.");
            }
        }
    }
}

impl UiActionParser {
    pub fn parse(&self) -> UiActionTokens {
        let mut tokens = UiActionTokens::default();

        for i in &self.fields {
            tokens.enum_variants.push(i.to_enum_variant());
            tokens.add_events.push(i.to_add_event());
            tokens.handler_args.push(i.to_handler_arg());
            tokens.handlers.push(i.to_handler());
        }

        tokens
    }
}

#[derive(Default)]
pub struct UiActionTokens {
    pub enum_variants: Vec<TokenStream2>,
    pub add_events: Vec<TokenStream2>,
    pub handler_args: Vec<TokenStream2>,
    pub handlers: Vec<TokenStream2>,
}

struct UiActionField {
    ident: Ident, // "create_todo"
    r#type: Type, // ["CreateTodo"]
}

impl From<Field> for UiActionField {
    fn from(f: Field) -> Self {
        Self {
            ident: f
                .clone()
                .ident
                .expect("Make sure to name each field in GlobalState struct"),
            r#type: f.ty,
        }
    }
}

impl UiActionField {
    // example: TodoList(Vec<UiTodo>),
    fn to_enum_variant(&self) -> TokenStream2 {
        let name_upper_camel =
            TokenStream2::from_str(&self.ident.to_string().to_case(Case::UpperCamel)).unwrap();
        let r#type = &self.r#type;

        quote! {
            #name_upper_camel(#r#type),
        }
    }

    // example: .add_event::<CreateTodo>()
    fn to_add_event(&self) -> TokenStream2 {
        let name_upper_camel =
            TokenStream2::from_str(&self.ident.to_string().to_case(Case::UpperCamel)).unwrap();

        quote! {
            .add_event::<#name_upper_camel>()
        }
    }

    // example: mut create_todo: EventWriter<CreateTodo>,
    fn to_handler_arg(&self) -> TokenStream2 {
        let name_snake =
            TokenStream2::from_str(&self.ident.to_string().to_case(Case::Snake)).unwrap();
        let name_upper_camel =
            TokenStream2::from_str(&self.ident.to_string().to_case(Case::UpperCamel)).unwrap();

        quote! {
            mut #name_snake: EventWriter<#name_upper_camel>,
        }
    }

    // example:
    // UiAction::CreateTodo(event) => {
    //     create_todo.send(event.clone());
    // }
    fn to_handler(&self) -> TokenStream2 {
        let name_snake =
            TokenStream2::from_str(&self.ident.to_string().to_case(Case::Snake)).unwrap();
        let name_upper_camel =
            TokenStream2::from_str(&self.ident.to_string().to_case(Case::UpperCamel)).unwrap();

        quote! {
            UiAction::#name_upper_camel(event) => {
                #name_snake.send(event.clone());
            }
        }
    }
}
