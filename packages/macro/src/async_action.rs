// use convert_case::{Case, Casing};
use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
// use std::str::FromStr;
use syn::ItemEnum;

pub struct AsyncActionParser {
    action_enum: ItemEnum,
}

impl AsyncActionParser {
    pub fn new(action_enum: ItemEnum) -> Self {
        Self { action_enum }
    }

    pub fn parse(&self) -> AsyncActionToken {
        // let action_enum = &self.action_enum;
        // let ident = &action_enum.ident;

        let token = AsyncActionToken {};

        token
    }
}

pub struct AsyncActionToken;

impl AsyncActionToken {
    pub fn gen(&self) -> TokenStream {
        let Self {} = self;

        let gen = quote! {
            pub struct AsyncActionPlugin;

            impl ::bevy::app::Plugin for AsyncActionPlugin {
                fn build(&self, app: &mut App) {
                    app.add_event::<GetIpAddress>()
                        .add_system_to_stage(::dip::core::schedule::UiStage::Action, handle_async_action);
                }
            }

            fn handle_async_action(
                mut events: EventReader<AsyncAction>,
                mut get_ip_address: EventWriter<GetIpAddress>,
            ) {
                for action in events.iter() {
                    match action {
                        AsyncAction::GetIpAddress(res) => {
                            get_ip_address.send(res.clone());
                        }
                    }
                }
            }
        };

        gen.into()
    }
}
