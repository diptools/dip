//! Dioxus Plugin for Bevy
#![allow(non_snake_case)]

use crate::{
    context::UiContext,
    event::{KeyboardEvent, UiEvent, VirtualDomCommand},
    event_loop::start_event_loop,
    setting::DioxusSettings,
    system::change_window,
    virtual_dom::VirtualDom,
    window::DioxusWindows,
};

use bevy::{
    app::{App, Plugin, CoreStage},
    ecs::{event::Events, world::World, schedule::ParallelSystemDescriptorCoercion},
    input::InputPlugin,
    window::{CreateWindow, ModifiesWindows, WindowCreated, WindowPlugin, Windows},
};
use bevy_dioxus_core::{global_state::GlobalStateHandler, schedule::UiSchedulePlugin};
use dioxus_core::{Component as DioxusComponent, SchedulerMsg};
use futures_channel::mpsc;
use futures_intrusive::channel::shared::channel;
use std::{fmt::Debug, marker::PhantomData, sync::Arc, sync::Mutex};
use tokio::runtime::Runtime;
use wry::application::event_loop::EventLoop;

/// Dioxus Plugin for Bevy
pub struct DioxusPlugin<GlobalState, CoreCommand, Props = ()> {
    /// Root component
    pub Root: DioxusComponent<Props>,

    global_state_type: PhantomData<GlobalState>,
    core_cmd_type: PhantomData<CoreCommand>,
}

impl<GlobalState, CoreCommand, Props> Plugin for DioxusPlugin<GlobalState, CoreCommand, Props>
where
    GlobalState: 'static + Send + Sync + GlobalStateHandler,
    CoreCommand: 'static + Send + Sync + Clone + Debug,
    Props: 'static + Send + Sync + Clone + Default,
{
    fn build(&self, app: &mut App) {
        let (vdom_scheduler_tx, vdom_scheduler_rx) = mpsc::unbounded::<SchedulerMsg>();
        let (vdom_command_tx, vdom_command_rx) = channel::<VirtualDomCommand<GlobalState>>(8);
        let (core_tx, core_rx) = channel::<CoreCommand>(8);

        let event_loop = EventLoop::<UiEvent<CoreCommand>>::with_user_event();
        let settings = app
            .world
            .remove_non_send_resource::<DioxusSettings<Props>>()
            .unwrap_or_default();

        let proxy = event_loop.create_proxy();
        let edit_queue = Arc::new(Mutex::new(Vec::new()));

        let runtime = Runtime::new().unwrap();

        let proxy_clone = proxy.clone();
        runtime.spawn(async move {
            while let Some(cmd) = core_rx.clone().receive().await {
                log::trace!("CoreCommand: {:#?}", cmd);
                proxy_clone.send_event(UiEvent::CoreCommand(cmd)).unwrap();
            }
        });

        let root_clone = self.Root.clone();
        let props_clone = settings.props.as_ref().unwrap().clone();
        let edit_queue_clone = edit_queue.clone();
        let vdom_scheduler_tx_clone = vdom_scheduler_tx.clone();
        app.add_plugin(WindowPlugin::default())
            .add_plugin(UiSchedulePlugin)
            .add_plugin(InputPlugin)
            .add_event::<KeyboardEvent>()
            .add_event::<CoreCommand>()
            .insert_resource(runtime)
            .insert_resource(vdom_scheduler_tx)
            .insert_resource(vdom_command_tx)
            .insert_resource(edit_queue)
            .init_non_send_resource::<DioxusWindows>()
            .insert_non_send_resource(settings)
            .insert_non_send_resource(event_loop)
            .set_runner(|app| start_event_loop::<CoreCommand, Props>(app))
            .add_system_to_stage(CoreStage::PostUpdate, change_window.label(ModifiesWindows));

        std::thread::spawn(move || {
            Runtime::new().unwrap().block_on(async move {
                let mut virtual_dom = VirtualDom::new(
                    root_clone,
                    props_clone,
                    edit_queue_clone,
                    (vdom_scheduler_tx_clone, vdom_scheduler_rx),
                    vdom_command_rx,
                );
                virtual_dom.provide_ui_context(UiContext::new(proxy.clone(), core_tx));
                virtual_dom.run().await;
            });
        });

        Self::handle_initial_window_events(&mut app.world);
    }
}

impl<GlobalState, CoreCommand, Props> DioxusPlugin<GlobalState, CoreCommand, Props>
where
    GlobalState: Send + Sync + GlobalStateHandler,
    CoreCommand: Clone + Debug + Send + Sync,
    Props: Send + Sync + Clone + 'static,
{
    /// Initialize DioxusPlugin with root component and channel types
    ///
    /// ```no_run
    /// use bevy_dioxus::desktop::prelude::*;
    ///
    /// // DioxusPlugin accepts any types as command. Pass empty tuple if channel is not necessary.
    /// type CoreCommand = ();
    ///
    /// fn main() {
    ///    App::new()
    ///         .add_plugin(DioxusPlugin::<EmptyGlobalState, CoreCommand>::new(Root))
    ///         .run();
    /// }
    ///
    /// fn Root(cx: Scope) -> Element {
    ///    cx.render(rsx! {
    ///    h1 { "Hello, World !" }
    ///        })
    /// }
    /// ```
    pub fn new(Root: DioxusComponent<Props>) -> Self {
        Self {
            Root,
            core_cmd_type: PhantomData,
            global_state_type: PhantomData,
        }
    }

    fn handle_initial_window_events(world: &mut World)
    where
        CoreCommand: 'static + Send + Sync + Clone + Debug,
        Props: 'static + Send + Sync + Clone,
    {
        let world = world.cell();
        let mut dioxus_windows = world.get_non_send_resource_mut::<DioxusWindows>().unwrap();
        let mut bevy_windows = world.get_resource_mut::<Windows>().unwrap();
        let mut create_window_events = world.get_resource_mut::<Events<CreateWindow>>().unwrap();
        let mut window_created_events = world.get_resource_mut::<Events<WindowCreated>>().unwrap();

        for create_window_event in create_window_events.drain() {
            let window = dioxus_windows.create::<CoreCommand, Props>(
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
