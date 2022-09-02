use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{FnArg, ImplItem, ImplItemMethod, ItemImpl, ReturnType, Type};

pub struct UiActionCreatorParser {
    action_creator_impl: ItemImpl,
}

impl From<ItemImpl> for UiActionCreatorParser {
    fn from(action_creator_impl: ItemImpl) -> Self {
        Self {
            action_creator_impl,
        }
    }
}

impl UiActionCreatorParser {
    pub fn parse(&self) -> UiActionCreatorTokenStreams {
        UiActionCreatorTokenStreams {
            action_creator_name: self.action_creator_name(),
            action_creator_impl: self.action_creator_impl(),
            methods: self.methods(),
        }
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
    fn methods(&self) -> Vec<TokenStream2> {
        let mut methods = vec![];
        let action_creator_name = self.action_creator_name();
        for i in self.action_creator_impl.clone().items {
            match i {
                ImplItem::Method(m) => {
                    let method_name = &m.sig.ident;
                    let event_name = match &m.sig.output {
                        ReturnType::Type(_, return_type) => match *return_type.clone() {
                            Type::Path(type_path) => type_path.path.segments[0].ident.clone(),
                            _ => {
                                panic!("Cannot find event name. Make sure to sepcify return event in action creator methods.");
                            }
                        },
                        _ => {
                            panic!("Cannot find event name. Make sure to sepcify return event in action creator methods.");
                        }
                    };

                    let (arg_keys, args) = Self::method_args(&m);

                    methods.push(quote! {
                        pub fn #method_name(#(#args)*) -> Self {
                            Self::#event_name(#action_creator_name::#method_name(#(#arg_keys)*))
                        }
                    });
                }
                _ => {}
            }
        }
        methods
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
pub struct UiActionCreatorTokenStreams {
    action_creator_name: TokenStream2,
    action_creator_impl: TokenStream2,
    methods: Vec<TokenStream2>,
}

impl UiActionCreatorTokenStreams {
    pub fn gen(&self) -> TokenStream2 {
        let Self {
            action_creator_name,
            action_creator_impl,
            methods,
        } = self;

        let gen = quote! {
            struct #action_creator_name;

            #action_creator_impl

            impl UiAction {
                #(#methods)*
            }
        };

        gen.into()
    }
}
