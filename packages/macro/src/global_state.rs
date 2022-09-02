use convert_case::{Case, Casing};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::{Data, DeriveInput, Field, Ident, Type};

pub struct GlobalStateParser {
    fields: Vec<GlobalStateField>,
}

impl From<DeriveInput> for GlobalStateParser {
    fn from(input: DeriveInput) -> Self {
        match input.data {
            Data::Struct(data) => {
                let mut fields = vec![];
                for f in data.fields {
                    fields.push(GlobalStateField::from(f));
                }
                Self { fields }
            }
            _ => {
                panic!("global_state macro can only be used for struct.");
            }
        }
    }
}

impl GlobalStateParser {
    pub fn parse(&self) -> GlobalStateTokens {
        let mut tokens = GlobalStateTokens::default();

        for i in &self.fields {
            tokens.atom_quotes.push(i.to_atom_quote());
            tokens.enum_variants.push(i.to_enum_variant());
            tokens.variant_handlers.push(i.to_variant_handler());
        }

        tokens
    }
}

#[derive(Default)]
pub struct GlobalStateTokens {
    pub atom_quotes: Vec<TokenStream2>,
    pub enum_variants: Vec<TokenStream2>,
    pub variant_handlers: Vec<TokenStream2>,
}

struct GlobalStateField {
    ident: Ident, // "todo_list"
    r#type: Type, // ["Vec<UiTodo>"]
}

impl From<Field> for GlobalStateField {
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

impl GlobalStateField {
    // example: pub static TODO_LIST: Atom<Vec<UiTodo>> = |_| Vec::default();
    fn to_atom_quote(&self) -> TokenStream2 {
        let name_upper_snake =
            TokenStream2::from_str(&self.ident.to_string().to_case(Case::UpperSnake)).unwrap();
        let r#type = self.r#type.clone();
        let type_name = match self.r#type.clone() {
            Type::Path(p) => p.path.segments[0].ident.clone(),
            _ => {
                panic!("Make sure GlobalState struct has right structure");
            }
        };

        quote! {
            pub static #name_upper_snake: Atom<#r#type> = |_| #type_name::default();
        }
    }

    // example: TodoList(Vec<UiTodo>),
    fn to_enum_variant(&self) -> TokenStream2 {
        let name_upper_camel =
            TokenStream2::from_str(&self.ident.to_string().to_case(Case::UpperCamel)).unwrap();
        let r#type = &self.r#type;

        quote! {
            #name_upper_camel(#r#type),
        }
    }

    // example: GlobalState::TodoList(x) => root.set(TODO_LIST.unique_id(), x),
    fn to_variant_handler(&self) -> TokenStream2 {
        let name_upper_camel =
            TokenStream2::from_str(&self.ident.to_string().to_case(Case::UpperCamel)).unwrap();
        let name_upper_snake =
            TokenStream2::from_str(&self.ident.to_string().to_case(Case::UpperSnake)).unwrap();

        quote! {
            GlobalState::#name_upper_camel(x) => root.set(#name_upper_snake.unique_id(), x),
        }
    }
}
