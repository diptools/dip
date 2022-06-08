use bevy::{log::LogPlugin, prelude::*};
use bevy_dioxus::desktop::prelude::*;
use dioxus::{
    fermi::{AtomRoot, Readable},
    prelude::*,
};
use std::rc::Rc;

fn main() {
    App::new()
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<CoreCommand, (), GlobalState>::new(Root))
        .add_startup_system(setup)
        .add_system(handle_core_cmd.label("handle-core-cmd"))
        .add_system(update_count_atom.after("handle-core-cmd"))
        .run();
}

#[derive(Component, Clone, Debug, Default)]
struct Count(pub u32);

#[derive(Component, Clone, Debug)]
struct Disabled(bool);

impl Default for Disabled {
    fn default() -> Self {
        Self(true)
    }
}

// TODO: derive by macro ?
static COUNT: Atom<Count> = |_| Count::default();
static DISABLED: Atom<Disabled> = |_| Disabled::default();

#[derive(Debug)]
enum GlobalState {
    Count(Count),
    Disabled(Disabled),
}

impl GlobalStateHandler<GlobalState> for GlobalState {
    fn handler(root: Rc<AtomRoot>, state: GlobalState) {
        match state {
            GlobalState::Count(count) => {
                root.set(COUNT.unique_id(), count);
            }
            GlobalState::Disabled(disabled) => {
                root.set(DISABLED.unique_id(), disabled);
            }
        }
    }
}

#[derive(Clone, Debug)]
enum CoreCommand {
    Increment,
    Decrement,
    Reset,
}

fn setup(mut commands: Commands) {
    info!("🧠 Spawn count");
    commands
        .spawn()
        .insert(Count::default())
        .insert(Disabled::default());
}

// TODO: should be derived by macro
fn update_count_atom(
    counts: Query<&Count, Changed<Count>>,
    disabled: Query<&Disabled, Changed<Disabled>>,
    vdom_tx: Res<Sender<VDomCommand<GlobalState>>>,
) {
    for count in counts.iter() {
        match vdom_tx.try_send(VDomCommand::GlobalState(GlobalState::Count(count.clone()))) {
            Ok(()) => {}
            Err(e) => match e {
                TrySendError::Full(e) => {
                    error!(
                        "Failed to send VDomCommand: channel is full: event: {:?}",
                        e
                    );
                }
                TrySendError::Closed(e) => {
                    error!(
                        "Failed to send VDomCommand: channel is closed: event: {:?}",
                        e
                    );
                }
            },
        }
    }
    for disabled in disabled.iter() {
        match vdom_tx.try_send(VDomCommand::GlobalState(GlobalState::Disabled(
            disabled.clone(),
        ))) {
            Ok(()) => {}
            Err(e) => match e {
                TrySendError::Full(e) => {
                    error!(
                        "Failed to send VDomCommand: channel is full: event: {:?}",
                        e
                    );
                }
                TrySendError::Closed(e) => {
                    error!(
                        "Failed to send VDomCommand: channel is closed: event: {:?}",
                        e
                    );
                }
            },
        }
    }
}

fn handle_core_cmd(
    mut events: EventReader<CoreCommand>,
    mut query: Query<(&mut Count, &mut Disabled)>,
) {
    for cmd in events.iter() {
        let (mut count, mut disabled) = query.single_mut();
        match cmd {
            CoreCommand::Increment => {
                info!("🧠 Increment");
                count.0 += 1;
            }
            CoreCommand::Decrement => {
                if count.0 > 0 {
                    info!("🧠 Decrement");
                    count.0 -= 1;
                }
            }
            CoreCommand::Reset => {
                if count.0 != 0 {
                    info!("🧠 Reset");
                    count.0 = 0;
                }
            }
        };
        disabled.0 = count.0 == 0;
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let window = use_window::<CoreCommand, ()>(&cx);
    let count = use_read(&cx, COUNT);
    let disabled = use_read(&cx, DISABLED);

    cx.render(rsx! {
        h1 { "Counter Example" }
        p { "count: {count.0}" }
        button {
            onclick: move |_| window.send(CoreCommand::Decrement),
            disabled: "{disabled.0}",
            "-",
        }
        button {
            onclick: move |_| window.send(CoreCommand::Reset),
            disabled: "{disabled.0}",
            "Reset"
        }
        button {
            onclick: move |_| window.send(CoreCommand::Increment),
            "+",
        }
    })
}
