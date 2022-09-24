use dip::{
    bevy::{
        log::{self, LogPlugin},
        time::TimePlugin,
    },
    prelude::*,
};

/// This example illustrates how to customize render setting
fn main() {
    App::new()
        .add_plugin(LogPlugin)
        .add_plugin(TimePlugin)
        .add_plugin(DesktopPlugin::<UiState, UiAction>::new(Root))
        .add_plugin(UiStatePlugin)
        .add_plugin(UiActionPlugin)
        .add_system(increment_frame)
        .add_system(update_render_mode)
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
            p { [format_args!("Frame: {}", frame.value)] }
        }
    })
}

#[ui_state]
struct UiState {
    frame: Frame,
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

#[derive(Clone, Debug, Default)]
pub struct Frame {
    value: u32,
}

#[derive(Clone, Debug)]
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

fn update_render_mode(
    mut actions: EventReader<RenderMode>,
    mut render_mode: ResMut<RenderMode>,
    mut desktop_settings: NonSendMut<DesktopSettings<NoRootProps>>,
) {
    for mode in actions.iter() {
        *render_mode = mode.clone();
        *desktop_settings = match mode {
            RenderMode::Application => DesktopSettings::application(),
            RenderMode::Game => DesktopSettings::game(),
        };
    }
}
