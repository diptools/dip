use bevy::{core::CorePlugin, log::LogPlugin, prelude::*};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;
use leafwing_input_manager::prelude::*;

/// This example illustrates how to customize render setting
fn main() {
    App::new()
        .add_plugin(CorePlugin)
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<(), UiCommand>::new(Root))
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(setup)
        .add_system(update_frame)
        .add_system(toggle_update_mode)
        .add_system(handle_mode_update)
        .run();
}

#[derive(Component)]
struct User;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    ToggleUpdateMode,
}

#[derive(Component, Clone, Debug, Default)]
struct Frame(u32);

#[derive(Component, Clone, Debug)]
enum Mode {
    Application,
    Game,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Application
    }
}

#[derive(Clone, Debug)]
enum UiCommand {
    Frame(u32),
    Mode(Mode),
}

fn setup(mut commands: Commands, mut ui: EventWriter<UiCommand>) {
    let frame = Frame::default();
    commands.spawn().insert(Frame::default());
    commands.spawn().insert(Mode::default());
    commands
        .spawn()
        .insert(User)
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::new([(Action::ToggleUpdateMode, KeyCode::Space)]),
        });

    ui.send(UiCommand::Frame(frame.0));
}

fn update_frame(mut query: Query<&mut Frame>, mut ui: EventWriter<UiCommand>) {
    let mut frame = query.single_mut();
    frame.0 += 1;
    ui.send(UiCommand::Frame(frame.0));
}

fn toggle_update_mode(query: Query<&ActionState<Action>, With<User>>, mut modes: Query<&mut Mode>) {
    let action_state = query.single();
    if action_state.just_pressed(Action::ToggleUpdateMode) {
        let mut mode = modes.single_mut();
        *mode = match *mode {
            Mode::Application => Mode::Game,
            Mode::Game => Mode::Application,
        };
    }
}

fn handle_mode_update(
    query: Query<&Mode, Changed<Mode>>,
    mut settings: NonSendMut<DioxusSettings<()>>,
    mut events: EventWriter<UiCommand>,
) {
    for mode in query.iter() {
        *settings = match *mode {
            Mode::Application => DioxusSettings::desktop_app(),
            Mode::Game => DioxusSettings::game(),
        };
        events.send(UiCommand::Mode(mode.clone()));
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let window = use_window::<(), UiCommand>(&cx);
    let frame = use_state(&cx, || 0);
    let mode = use_state(&cx, || Mode::Application);

    use_future(&cx, (), |_| {
        let rx = window.receiver();
        let frame = frame.clone();
        let mode = mode.clone();

        async move {
            while let Some(cmd) = rx.receive().await {
                match cmd {
                    UiCommand::Frame(f) => frame.modify(|_| f),
                    UiCommand::Mode(m) => mode.modify(|_| m),
                }
            }
        }
    });

    cx.render(rsx! {
        h1 { "Window: Update Mode" }
        p { "💡 Press \"Space\" to toggle mode. (TODO: You might need to click screen to focus.)" }
        p { "The frame gets updated only when user event occurs or with max timeout on Application mode." }
        p { "Try press any keys or change window size to see frame increments." }
        div {
            style: "background: #ddd; padding: 1rem;",
            p { [format_args!("Mode: {:?}", mode)] }
            p { [format_args!("Frame: {}", frame)] }
        }
    })
}
