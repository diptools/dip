use crate::context::UiContext;
use dioxus_core::*;
use std::fmt::Debug;

pub fn use_window<CoreCommand, UiCommand>(cx: &ScopeState) -> &UiContext<CoreCommand, UiCommand>
where
    CoreCommand: Debug + Clone,
    UiCommand: Clone + 'static,
{
    cx.use_hook(|_| cx.consume_context::<UiContext<CoreCommand, UiCommand>>())
        .as_ref()
        .expect("Failed to find UiContext, check CoreCommand and UiCommand type parameter")
}
