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

/// Get a closure that executes any JavaScript in the WebView context.
pub fn use_eval<CoreCommand, UiCommand, S: std::string::ToString>(cx: &ScopeState) -> &dyn Fn(S)
where
    CoreCommand: Debug + Clone + 'static,
    UiCommand: Clone + 'static + Debug,
{
    let window = use_window::<CoreCommand, UiCommand>(cx).clone();

    cx.use_hook(|_| move |script| window.eval(script))
}
