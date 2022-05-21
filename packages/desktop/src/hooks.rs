use crate::context::UiContext;
use dioxus_core::*;
use std::fmt::Debug;

pub fn use_bevy_window<CoreCommand, UiCommand>(
    cx: &ScopeState,
) -> &UiContext<CoreCommand, UiCommand>
where
    CoreCommand: Debug + Clone,
    UiCommand: Clone + 'static,
{
    cx.use_hook(|_| cx.consume_context::<UiContext<CoreCommand, UiCommand>>())
        .as_ref()
        .unwrap()
}

// TODO
// pub fn use_bevy_listener<CoreCommand, UiCommand, D, F>(
//     cx: &ScopeState,
//     deps: D,
//     handler: impl Fn(UiCommand, D::Out) -> F + 'static,
// ) -> &UseFuture<()>
// where
//     CoreCommand: Debug + Clone + 'static,
//     UiCommand: Clone + 'static,
//     F: Future<Output = ()> + 'static,
//     D: UseFutureDep + 'static,
//     <D as UseFutureDep>::Out: Clone,
// {
//     let ctx = use_bevy_window::<CoreCommand, UiCommand>(&cx);

//     let state = use_future(&cx, deps, |deps| {
//         let mut rx = ctx.receiver();

//         async move {
//             while let Ok(cmd) = rx.recv().await {
//                 handler(cmd, deps);
//             }
//         }
//     });

//     state
// }
