use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use std::str::FromStr;
use syn::{Field, Fields, ItemEnum, Variant};

pub struct SubcommandParser {
    commands_enum: ItemEnum,
}

impl SubcommandParser {
    pub fn new(commands_enum: ItemEnum) -> Self {
        Self { commands_enum }
    }

    pub fn parse(&self) -> SubcommandToken {
        let mut tokens = SubcommandToken {
            handler_name: self.handler_name(),
            plugin_name: self.plugin_name(),
            subcommand_ty_name: self.subcommand_ty_name(),
            add_system: self.add_system(),
            ..Default::default()
        };

        for v in &self.commands_enum.variants {
            tokens.events.push(self.event(&v));
            tokens.add_events.push(self.add_event(&v));
            tokens.event_readers.push(self.event_reader(&v));
            tokens.handlers.push(self.handler(&v));
        }

        tokens
    }

    fn plugin_name(&self) -> TokenStream2 {
        TokenStream2::from_str(&format!("{}Plugin", &self.commands_enum.ident.to_string())).unwrap()
    }

    fn handler_name(&self) -> TokenStream2 {
        let ty = &self.commands_enum.ident.to_string().to_case(Case::Snake);
        TokenStream2::from_str(&format!("handle_{}", ty)).unwrap()
    }

    fn add_system(&self) -> TokenStream2 {
        let handler_with_system_order = self.handler_with_system_order();

        let gen = quote! {
            .add_startup_system_to_stage(::dip::core::schedule::DipStartupStage::Action, #handler_with_system_order);
        };

        gen
    }

    fn handler_with_system_order(&self) -> TokenStream2 {
        let gen_str = self
            .child_names()
            .iter()
            .map(|name| format!("handle_{}", name.to_case(Case::Snake)))
            .fold(self.handler_name().to_string(), |acc, name| {
                format!("{acc}.before({})", name)
            });
        let gen = TokenStream2::from_str(&gen_str).unwrap();

        gen
    }

    fn child_names(&self) -> Vec<String> {
        let mut child_names = vec![];
        for v in self.commands_enum.variants.iter() {
            for a in v.attrs.iter() {
                for t in a.tokens.clone().into_iter() {
                    match t {
                        TokenTree::Group(g) => {
                            for s in g.stream() {
                                match s {
                                    TokenTree::Ident(ident) => {
                                        if ident.to_string() == "subcommand" {
                                            if let syn::Fields::Unnamed(f) = &v.fields {
                                                let child_ty = &f.unnamed[0].ty;
                                                let child_ty_quote = quote! { #child_ty };
                                                let child_name = child_ty_quote
                                                    .to_string()
                                                    .to_case(Case::UpperCamel);

                                                child_names.push(child_name);
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        child_names
    }

    fn subcommand_ty_name(&self) -> TokenStream2 {
        let ident = &self.commands_enum.ident;
        quote! { #ident }
    }

    fn event(&self, v: &Variant) -> TokenStream2 {
        let name = self.event_name(v);

        match &v.fields {
            Fields::Named(f) => {
                let mut fields = vec![];
                for Field { ident, ty, .. } in f.named.iter() {
                    fields.push(quote! {
                        pub #ident: #ty,
                    });
                }

                quote! {
                    #[derive(Clone, Debug)]
                    pub struct #name {
                        #(#fields)*
                    }
                }
            }
            Fields::Unnamed(f) => {
                let first_field_ty = &f.unnamed.first().unwrap().ty;

                // add event name alias only when type name is different (if variant_ident != first_field_ty)
                if &name.to_string() == quote! { #first_field_ty }.to_string().as_str() {
                    quote! {}
                } else {
                    quote! { pub type #name = #first_field_ty; }
                }
            }
            Fields::Unit => {
                quote! {
                    #[derive(Clone, Debug)]
                    pub struct #name;
                }
            }
        }
    }

    fn add_event(&self, v: &Variant) -> TokenStream2 {
        let name = self.event_name(v);

        quote! { .add_event::<#name>() }
    }

    fn event_name(&self, v: &Variant) -> TokenStream2 {
        TokenStream2::from_str(&format!(
            "{}{}",
            &v.ident.to_string(),
            &self.subcommand_ty_name().to_string(),
        ))
        .unwrap()
    }

    fn event_reader(&self, v: &Variant) -> TokenStream2 {
        let name = self.event_name(v);
        let event_name_snake =
            TokenStream2::from_str(&name.to_string().to_case(Case::Snake)).unwrap();

        quote! { mut #event_name_snake: ::dip::bevy::ecs::event::EventWriter<#name>, }
    }

    fn handler(&self, v: &Variant) -> TokenStream2 {
        let subcommand_ty_name = self.subcommand_ty_name();
        let event_name = self.event_name(v);
        let ident = &v.ident;
        let name = self.event_name(v);
        let event_name_snake =
            TokenStream2::from_str(&name.to_string().to_case(Case::Snake)).unwrap();

        match &v.fields {
            Fields::Named(fields) => {
                let mut field_names = vec![];
                let mut new_fields = vec![];
                for f in &fields.named {
                    let field_name = f.ident.clone().unwrap();
                    let new_field =
                        TokenStream2::from_str(&format!("{field_name}: {field_name}.clone()"))
                            .unwrap();

                    field_names.push(field_name);
                    new_fields.push(new_field)
                }

                quote! {
                    #subcommand_ty_name::#ident { #(#field_names)*, } => {
                        #event_name_snake.send(#event_name { #(#new_fields)*, });
                    }
                }
            }
            Fields::Unnamed(_f) => {
                quote! {
                    #subcommand_ty_name::#ident(x) => {
                        #event_name_snake.send(x.clone());
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    #subcommand_ty_name::#ident => {
                        #event_name_snake.send(#event_name);
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SubcommandToken {
    handler_name: TokenStream2,
    plugin_name: TokenStream2,
    subcommand_ty_name: TokenStream2,
    events: Vec<TokenStream2>,
    add_events: Vec<TokenStream2>,
    event_readers: Vec<TokenStream2>,
    handlers: Vec<TokenStream2>,
    add_system: TokenStream2,
}

impl SubcommandToken {
    pub fn gen(&self) -> TokenStream {
        let Self {
            handler_name,
            plugin_name,
            subcommand_ty_name,
            events,
            add_events,
            event_readers,
            handlers,
            add_system,
        } = self;

        let gen = quote! {
            pub struct #plugin_name;

            impl ::dip::bevy::app::Plugin for #plugin_name {
                fn build(&self, app: &mut ::dip::bevy::app::App) {
                    use ::dip::bevy::ecs::{
                        schedule::ParallelSystemDescriptorCoercion,
                        system::IntoSystem,
                    };

                    app #(#add_events)*
                        #add_system
                }
            }

            #(#events)*

            pub fn #handler_name(
                mut events: ::dip::bevy::ecs::event::EventReader<#subcommand_ty_name>,
                #(#event_readers)*
            ) {
                for e in events.iter() {
                    match e {
                        #(#handlers)*
                    }
                }
            }
        };

        gen.into()
    }
}
