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
        .add_plugin(DioxusPlugin::<UiState, UiAction>::new(Root))
        .add_plugin(UiStatePlugin)
        .init_resource::<Frame>()
        .init_resource::<RenderMode>()
        .add_system(increment_frame)
        .add_system(update_render_mode)
        .add_system_to_stage(UiStage::Prepare, update_ui_state)
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let window = use_window::<UiAction>(&cx);

    let frame = use_read(&cx, FRAME);
    let render_mode = use_read(&cx, RENDER_MODE);

    cx.render(rsx! {
        h1 { "Window: Render Mode" }

        select {
            value: format_args!("{:?}", render_mode),
            onchange: |e| {
                log::trace!("onchange: {:#?}", e.value.as_str());
                match e.value.as_str() {
                    "Application" => { window.send(UiAction::application()) }
                    "Game" => { window.send(UiAction::game()) }
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

#[ui_state]
struct UiState {
    frame: u32,
    render_mode: RenderMode,
}

#[ui_action]
impl ActionCreator {
    fn application() -> RenderMode {
        RenderMode::Application
    }

    fn game() -> RenderMode {
        RenderMode::Game
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

fn update_ui_state(
    frame: Res<Frame>,
    render_mode: Res<RenderMode>,
    mut ui_state: EventWriter<UiState>,
) {
    if frame.is_changed() {
        ui_state.send(UiState::Frame(frame.value));
    }

    if render_mode.is_changed() {
        ui_state.send(UiState::RenderMode(render_mode.clone()));
    }
}

fn update_render_mode(
    mut events: EventReader<UiAction>,
    mut render_mode: ResMut<RenderMode>,
    mut dioxus_settings: NonSendMut<DioxusSettings<()>>,
) {
    for action in events.iter() {
        match action {
            UiAction::RenderMode(mode) => {
                *render_mode = mode.clone();
                *dioxus_settings = match mode {
                    RenderMode::Application => DioxusSettings::application(),
                    RenderMode::Game => DioxusSettings::game(),
                }
            }
        }
    }
}
