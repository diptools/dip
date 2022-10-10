//! Dioxus Plugin for Bevy
#![allow(non_snake_case)]

use crate::{
    context::UiContext,
    event::{KeyboardEvent, UiEvent},
    event_loop::start_event_loop,
    setting::DesktopSettings,
    system::change_window,
    virtual_dom::VirtualDom,
    window::DioxusWindows,
};

use bevy::{
    app::{App, CoreStage, Plugin},
    ecs::{event::Events, schedule::ParallelSystemDescriptorCoercion, world::World},
    input::InputPlugin,
    window::{CreateWindow, ModifiesWindows, WindowCreated, WindowPlugin, Windows},
};
use dioxus_core::{Component as DioxusComponent, SchedulerMsg};
use dip_core::{schedule::UiSchedulePlugin, task::AsyncActionPool, ui_state::UiStateHandler};
use futures_channel::mpsc as futures_mpsc;
use std::{fmt::Debug, marker::PhantomData, sync::Arc, sync::Mutex};
use tokio::{runtime::Runtime, select, sync::mpsc};
use wry::application::event_loop::EventLoop;

/// Dioxus Plugin for Bevy
pub struct DesktopPlugin<UiState, UiAction, AsyncAction, RootProps = ()> {
    /// Root component
    pub Root: DioxusComponent<RootProps>,

    ui_state_type: PhantomData<UiState>,
    ui_action_type: PhantomData<UiAction>,
    async_action_type: PhantomData<AsyncAction>,
}

impl<UiState, UiAction, AsyncAction, RootProps> Plugin
    for DesktopPlugin<UiState, UiAction, AsyncAction, RootProps>
where
    UiState: 'static + Send + Sync + UiStateHandler,
    UiAction: 'static + Send + Sync + Clone + Debug,
    AsyncAction: 'static + Send + Sync + Clone + Debug,
    RootProps: 'static + Send + Sync + Clone + Default,
{
    fn build(&self, app: &mut App) {
        let (vdom_scheduler_tx, vdom_scheduler_rx) = futures_mpsc::unbounded::<SchedulerMsg>();
        let (ui_state_tx, ui_state_rx) = mpsc::channel::<UiState>(8);
        let (ui_action_tx, mut ui_action_rx) = mpsc::channel::<UiAction>(8);
        let (async_action_tx, mut async_action_rx) = mpsc::channel::<AsyncAction>(8);
        let async_action = AsyncActionPool::new(async_action_tx.clone());

        let event_loop = EventLoop::<UiEvent<UiAction, AsyncAction>>::with_user_event();
        let settings = app
            .world
            .remove_non_send_resource::<DesktopSettings<RootProps>>()
            .unwrap_or_default();

        let proxy = event_loop.create_proxy();
        let edit_queue = Arc::new(Mutex::new(Vec::new()));

        let runtime = Runtime::new().unwrap();

        let proxy_clone = proxy.clone();
        runtime.spawn(async move {
            loop {
                select! {
                    action = ui_action_rx.recv() => {
                        log::trace!("UiAction: {:#?}", action);
                        proxy_clone.send_event(UiEvent::UiAction(action.unwrap())).unwrap();
                    }
                    action = async_action_rx.recv() => {
                        log::trace!("AsyncAction: {:#?}", action);
                        proxy_clone.send_event(UiEvent::AsyncAction(action.unwrap())).unwrap();
                    }
                }
            }
        });

        let root_clone = self.Root.clone();
        let root_props_clone = settings.root_props.as_ref().unwrap().clone();
        let edit_queue_clone = edit_queue.clone();
        let vdom_scheduler_tx_clone = vdom_scheduler_tx.clone();
        app.add_plugin(WindowPlugin::default())
            .add_plugin(UiSchedulePlugin)
            .add_plugin(InputPlugin)
            .add_event::<KeyboardEvent>()
            .insert_resource(async_action)
            .insert_resource(runtime)
            .insert_resource(vdom_scheduler_tx)
            .insert_resource(ui_state_tx)
            .insert_resource(edit_queue)
            .init_non_send_resource::<DioxusWindows>()
            .insert_non_send_resource(settings)
            .insert_non_send_resource(event_loop)
            .set_runner(|app| start_event_loop::<UiAction, AsyncAction, RootProps>(app))
            .add_system_to_stage(CoreStage::PostUpdate, change_window.label(ModifiesWindows));

        std::thread::spawn(move || {
            Runtime::new().unwrap().block_on(async move {
                let mut virtual_dom = VirtualDom::new(
                    root_clone,
                    root_props_clone,
                    edit_queue_clone,
                    (vdom_scheduler_tx_clone, vdom_scheduler_rx),
                    ui_state_rx,
                );
                virtual_dom.provide_ui_context(UiContext::new(proxy.clone(), ui_action_tx));

                virtual_dom.run().await;
            });
        });

        Self::handle_initial_window_events(&mut app.world);
    }
}

impl<UiState, UiAction, AsyncAction, RootProps>
    DesktopPlugin<UiState, UiAction, AsyncAction, RootProps>
where
    UiState: Send + Sync + UiStateHandler,
    UiAction: Clone + Debug + Send + Sync,
    RootProps: Send + Sync + Clone + 'static,
{
    /// Initialize DioxusPlugin with root component and channel types
    ///
    /// ```no_run
    /// use dip::prelude::*;
    ///
    /// fn main() {
    ///    App::new()
    ///         .add_plugin(DesktopPlugin::<NoUiState, NoUiAction, NoAsyncAction>::new(Root))
    ///         .run();
    /// }
    ///
    /// fn Root(cx: Scope) -> Element {
    ///    cx.render(rsx! {
    ///    h1 { "Hello, World !" }
    ///        })
    /// }
    /// ```
    pub fn new(Root: DioxusComponent<RootProps>) -> Self {
        Self {
            Root,
            ui_state_type: PhantomData,
            ui_action_type: PhantomData,
            async_action_type: PhantomData,
        }
    }

    fn handle_initial_window_events(world: &mut World)
    where
        UiAction: 'static + Send + Sync + Clone + Debug,
        AsyncAction: 'static + Send + Sync + Clone + Debug,
        RootProps: 'static + Send + Sync + Clone,
    {
        let world = world.cell();
        let mut dioxus_windows = world.get_non_send_resource_mut::<DioxusWindows>().unwrap();
        let mut bevy_windows = world.get_resource_mut::<Windows>().unwrap();
        let mut create_window_events = world.get_resource_mut::<Events<CreateWindow>>().unwrap();
        let mut window_created_events = world.get_resource_mut::<Events<WindowCreated>>().unwrap();

        for create_window_event in create_window_events.drain() {
            let window = dioxus_windows.create::<UiAction, AsyncAction, RootProps>(
                &world,
                create_window_event.id,
                &create_window_event.descriptor,
            );
            bevy_windows.add(window);
            window_created_events.send(WindowCreated {
                id: create_window_event.id,
            });
        }
    }
}
