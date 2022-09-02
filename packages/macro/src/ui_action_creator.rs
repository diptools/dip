use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{FnArg, ImplItem, ImplItemMethod, ItemImpl, ReturnType, Type};

pub struct UiActionCreatorParser {
    action_creator_impl: ItemImpl,
    methods: Vec<ImplItemMethod>,
}

impl From<ItemImpl> for UiActionCreatorParser {
    fn from(action_creator_impl: ItemImpl) -> Self {
        let mut methods = vec![];
        for i in action_creator_impl.clone().items {
            match i {
                ImplItem::Method(method) => methods.push(method),
                _ => {}
            }
        }

        Self {
            action_creator_impl,
            methods,
        }
    }
}

impl UiActionCreatorParser {
    pub fn action_creator_impl(&self) -> TokenStream2 {
        let input = &self.action_creator_impl;
        quote! { #input }
    }

    // example
    // pub fn create_todo(title: &String) -> Self {
    //     Self::CreateTodo(ActionCreator::create_todo(title))
    // }
    pub fn methods(&self) -> Vec<TokenStream2> {
        let mut methods = vec![];
        for m in self.methods.iter() {
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

            let (arg_keys, args) = Self::method_args(m);

            methods.push(quote! {
                pub fn #method_name(#(#args)*) -> Self {
                    Self::#event_name(ActionCreator::#method_name(#(#arg_keys)*))
                }
            });
        }
        methods
    }

    pub fn method_args(method: &ImplItemMethod) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
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
