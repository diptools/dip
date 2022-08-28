extern crate proc_macro;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::{parse_macro_input, Data, DeriveInput, Field, Ident, Type};

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
            bevy::{app::Plugin, ecs::system::Res, log::error},
            desktop::{
                futures_intrusive::channel::{shared::Sender, TrySendError},
                event::VirtualDomCommand,
            },
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
                    .add_system(apply_global_state_command);
            }
        }

        fn apply_global_state_command(
            mut events: EventReader<GlobalState>,
            vdom_tx: Res<Sender<VirtualDomCommand<GlobalState>>>,
        ) {
            for e in events.iter() {
                match vdom_tx.try_send(VirtualDomCommand::GlobalState(e.clone())) {
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

struct GlobalStateParser {
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
                panic!("GlobalState derive macro can only be used for struct.");
            }
        }
    }
}

impl GlobalStateParser {
    fn parse(&self) -> GlobalStateTokens {
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
struct GlobalStateTokens {
    atom_quotes: Vec<TokenStream2>,
    enum_variants: Vec<TokenStream2>,
    variant_handlers: Vec<TokenStream2>,
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
