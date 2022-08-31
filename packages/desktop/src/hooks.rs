//! Provides utility hooks

use crate::context::UiContext;
use dioxus_core::*;
use std::fmt::Debug;

/// Get an imperative handle to the current window
pub fn use_window<UiAction>(cx: &ScopeState) -> &UiContext<UiAction>
where
    UiAction: Debug + Clone,
{
    cx.use_hook(|_| cx.consume_context::<UiContext<UiAction>>())
        .as_ref()
        .expect("Failed to find UiContext, check UiAction type parameter")
}

/// Get a closure that executes any JavaScript in the WebView context.
pub fn use_eval<UiAction, S: std::string::ToString>(cx: &ScopeState) -> &dyn Fn(S)
where
    UiAction: Debug + Clone + 'static,
{
    let window = use_window::<UiAction>(cx).clone();

    cx.use_hook(|_| move |script| window.eval(script))
}
