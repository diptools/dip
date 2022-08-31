use bevy_dioxus::{
    bevy::{
        log::{self, LogPlugin},
        time::TimePlugin,
    },
    desktop::prelude::*,
};

/// This example illustrates how to customize render setting
fn main() {
    App::new()
        .add_plugin(LogPlugin)
        .add_plugin(TimePlugin)
        .add_plugin(DioxusPlugin::<GlobalState, CoreCommand>::new(Root))
        .add_plugin(GlobalStatePlugin)
        // .insert_non_send_resource(DioxusSettings::<()>::game())
        .init_resource::<Frame>()
        .init_resource::<RenderMode>()
        .add_system(increment_frame)
        .add_system(handle_global_state_change)
        .add_system(update_render_mode)
        .run();
}

#[global_state]
struct GlobalState {
    frame: u32,
    render_mode: RenderMode,
}

#[derive(Clone, Debug)]
enum CoreCommand {
    RenderMode(RenderMode),
}

impl CoreCommand {
    fn application() -> Self {
        Self::RenderMode(RenderMode::Application)
    }

    fn game() -> Self {
        Self::RenderMode(RenderMode::Game)
    }
}

#[derive(Default)]
struct Frame {
    value: u32,
}

#[derive(Component, Clone, Debug)]
pub enum RenderMode {
    Application,
    Game,
}

impl Default for RenderMode {
    fn default() -> Self {
        Self::Application
    }
}

fn increment_frame(mut frame: ResMut<Frame>) {
    frame.value += 1;
    log::trace!("update_frame system: frame: {}", frame.value);
}

fn handle_global_state_change(
    frame: Res<Frame>,
    render_mode: Res<RenderMode>,
    mut global_state: EventWriter<GlobalState>,
) {
    if frame.is_changed() {
        global_state.send(GlobalState::Frame(frame.value));
    }

    if render_mode.is_changed() {
        global_state.send(GlobalState::RenderMode(render_mode.clone()));
    }
}

fn update_render_mode(
    mut events: EventReader<CoreCommand>,
    mut render_mode: ResMut<RenderMode>,
    mut dioxus_settings: NonSendMut<DioxusSettings<()>>,
    mut global_state: EventWriter<GlobalState>,
) {
    for cmd in events.iter() {
        match cmd {
            CoreCommand::RenderMode(mode) => {
                *render_mode = mode.clone();
                *dioxus_settings = match mode {
                    RenderMode::Application => DioxusSettings::application(),
                    RenderMode::Game => DioxusSettings::game(),
                }
            }
        }

        global_state.send(GlobalState::RenderMode(render_mode.clone()));
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let window = use_window::<CoreCommand>(&cx);

    let frame = use_read(&cx, FRAME);
    let render_mode = use_read(&cx, RENDER_MODE);

    cx.render(rsx! {
        h1 { "Window: Render Mode" }

        select {
            value: format_args!("{:?}", render_mode),
            onchange: |e| {
                log::trace!("onchange: {:#?}", e.value.as_str());
                match e.value.as_str() {
                    "Application" => { window.send(CoreCommand::application()) }
                    "Game" => { window.send(CoreCommand::game()) }
                    _ => {}
                }
            },
            option {
                value: "Application",
                "Application"
            }
            option {
                value: "Game",
                "Game"
            }
        }

        div {
            style: "background: #ddd; padding: 1rem;",
            p { [format_args!("Mode: {:?}", render_mode)] }
            p { [format_args!("Frame: {}", frame)] }
        }
    })
}
