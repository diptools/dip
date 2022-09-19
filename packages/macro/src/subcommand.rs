use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use std::str::FromStr;
use syn::{Fields, ItemEnum, Variant};

pub struct SubcommandParser {
    commands_enum: ItemEnum,
}

impl SubcommandParser {
    pub fn new(commands_enum: ItemEnum) -> Self {
        Self { commands_enum }
    }

    pub fn parse(&self) -> SubcommandTokenStreams {
        let mut tokens = SubcommandTokenStreams {
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
        let mut subsubcommand_handler_names = vec![];
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
                                                let subsubcommand_ty = &f.unnamed[0].ty;
                                                let subsubcommand_ty_quote =
                                                    quote! { #subsubcommand_ty };
                                                let subsubcommand_name = &subsubcommand_ty_quote
                                                    .to_string()
                                                    .to_case(Case::Snake);

                                                subsubcommand_handler_names
                                                    .push(format!("handle_{}", subsubcommand_name));
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

        let mut handler = self.handler_name().to_string();
        for n in subsubcommand_handler_names {
            handler = format!("{}.before({})", handler, n);
        }

        let handler_token = TokenStream2::from_str(&handler).unwrap();

        quote! {
            .add_system_to_stage(::dip::core::schedule::UiStage::Action, #handler_token);
        }
    }

    fn subcommand_ty_name(&self) -> TokenStream2 {
        let ident = &self.commands_enum.ident;
        quote! { #ident }
    }

    fn event(&self, v: &Variant) -> TokenStream2 {
        let name = self.event_name(v);

        match &v.fields {
            Fields::Named(f) => {
                quote! {
                    #[derive(Clone, Debug)]
                    pub struct #name #f
                }
            }
            Fields::Unnamed(_f) => {
                // let ty = &f.unnamed.first().unwrap().ty;
                // quote! { pub type #name = #ty; }
                quote! {}
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
        let ident_str = &v.ident.to_string();
        let name = match &v.fields {
            Fields::Named(f) => {
                println!("{:#?}", &f);
                // quote! {
                //     #[derive(Clone, Debug)]
                //     pub struct #name #f
                // }
                ""
            }
            Fields::Unnamed(_f) => {
                // let ty = &f.unnamed.first().unwrap().ty;
                // quote! { pub type #name = #ty; }
                ""
            }
            Fields::Unit => &ident_str,
        };
        println!("{name}");

        TokenStream2::from_str(&format!(
            "{}{}",
            &name,
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
                let mut names = vec![];
                for f in &fields.named {
                    names.push(f.ident.clone().unwrap());
                }

                quote! {
                    #subcommand_ty_name::#ident { #(#names)*, } => {
                        #event_name_snake.send(#event_name { #(#names)*, });
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
pub struct SubcommandTokenStreams {
    handler_name: TokenStream2,
    plugin_name: TokenStream2,
    subcommand_ty_name: TokenStream2,
    events: Vec<TokenStream2>,
    add_events: Vec<TokenStream2>,
    event_readers: Vec<TokenStream2>,
    handlers: Vec<TokenStream2>,
    add_system: TokenStream2,
}

impl SubcommandTokenStreams {
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
            #(#events)*

            pub struct #plugin_name;

            impl ::bevy::app::Plugin for #plugin_name {
                fn build(&self, app: &mut ::bevy::app::App) {
                    use ::dip::bevy::ecs::{
                        schedule::ParallelSystemDescriptorCoercion,
                        system::IntoSystem,
                    };

                    app #(#add_events)*
                        #add_system
                }
            }

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

        println!("{gen}");

        gen.into()
    }
}
