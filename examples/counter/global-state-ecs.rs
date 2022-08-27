use bevy::{log::LogPlugin, prelude::*};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

fn main() {
    App::new()
        .add_plugin(LogPlugin)
        .add_plugin(GlobalStatePlugin)
        .add_plugin(DioxusPlugin::<GlobalState, CoreCommand, ()>::new(Root))
        .add_event::<UpdateGlobalState>()
        .add_startup_system(setup)
        .add_system(handle_core_cmd)
        .add_system(update_global_state)
        .run();
}

#[global_state]
struct GlobalState {
    count: u32,
    disabled: bool,
}

#[derive(Component, Clone, Debug, Default)]
struct Count(u32);

#[derive(Component, Clone, Debug)]
struct Disabled(bool);

impl Default for Disabled {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Clone, Debug)]
enum CoreCommand {
    Increment,
    Decrement,
    Reset,
}

struct UpdateGlobalState;

fn setup(mut commands: Commands, mut update_global_state: EventWriter<UpdateGlobalState>) {
    info!("🧠 Spawn count");
    commands
        .spawn()
        .insert(Count::default())
        .insert(Disabled::default());

    update_global_state.send(UpdateGlobalState);
}

fn handle_core_cmd(
    mut events: EventReader<CoreCommand>,
    mut query: Query<(&mut Count, &mut Disabled)>,
    mut update_global_state: EventWriter<UpdateGlobalState>,
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

        update_global_state.send(UpdateGlobalState);
    }
}

fn update_global_state(
    mut events: EventReader<UpdateGlobalState>,
    query: Query<(&Count, &Disabled)>,
    mut global_state: EventWriter<GlobalState>,
) {
    for _ in events.iter() {
        let (count, disabled) = query.single();

        global_state.send(GlobalState::Count(count.0));
        global_state.send(GlobalState::Disabled(disabled.0));
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let count = use_read(&cx, COUNT);
    let disabled = use_read(&cx, DISABLED);

    let window = use_window::<CoreCommand>(&cx);

    cx.render(rsx! {
        h1 { "Counter Example" }
        p { "count: {count}" }
        button {
            onclick: move |_| window.send(CoreCommand::Decrement),
            disabled: "{disabled}",
            "-",
        }
        button {
            onclick: move |_| window.send(CoreCommand::Reset),
            disabled: "{disabled}",
            "Reset"
        }
        button {
            onclick: move |_| window.send(CoreCommand::Increment),
            "+",
        }
    })
}
