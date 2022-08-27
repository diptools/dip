//! Provides utility hooks

use crate::context::UiContext;
use dioxus_core::*;
use std::fmt::Debug;

/// Get an imperative handle to the current window
pub fn use_window<CoreCommand>(cx: &ScopeState) -> &UiContext<CoreCommand>
where
    CoreCommand: Debug + Clone,
{
    cx.use_hook(|_| cx.consume_context::<UiContext<CoreCommand>>())
        .as_ref()
        .expect("Failed to find UiContext, check CoreCommand type parameter")
}

/// Get a closure that executes any JavaScript in the WebView context.
pub fn use_eval<CoreCommand, S: std::string::ToString>(cx: &ScopeState) -> &dyn Fn(S)
where
    CoreCommand: Debug + Clone + 'static,
{
    let window = use_window::<CoreCommand>(cx).clone();

    cx.use_hook(|_| move |script| window.eval(script))
}
