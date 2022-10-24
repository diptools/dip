//! Provides utility hooks

use crate::context::UiContext;
use dioxus_core::*;
use std::fmt::Debug;

/// Get an imperative handle to the current window
pub fn use_window<UiAction, AsyncAction>(cx: &ScopeState) -> &UiContext<UiAction, AsyncAction>
where
    UiAction: Debug + Clone,
    AsyncAction: Debug + Clone,
{
    cx.use_hook(|_| cx.consume_context::<UiContext<UiAction, AsyncAction>>())
        .as_ref()
        .expect("Failed to find UiContext, check UiAction type parameter")
}

/// Get a closure that executes any JavaScript in the WebView context.
pub fn use_eval<UiAction, AsyncAction, S: std::string::ToString>(cx: &ScopeState) -> &dyn Fn(S)
where
    UiAction: Debug + Clone + 'static,
    AsyncAction: Debug + Clone + 'static,
{
    let window = use_window::<UiAction, AsyncAction>(cx).clone();

    cx.use_hook(|_| move |script| window.eval(script))
}
