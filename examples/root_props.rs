use dip::{bevy::log::LogPlugin, desktop::prelude::*};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Props Example".to_string(),
            ..Default::default()
        })
        .add_plugin(DesktopPlugin::<NoUiState, NoUiAction, RootProps>::new(Root))
        .add_plugin(LogPlugin)
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope<RootProps>) -> Element {
    cx.render(rsx! {
        h1 { "Hello, {cx.props.name} !" }
    })
}

#[derive(Props, PartialEq, Clone)]
struct RootProps {
    name: String,
}

impl Default for RootProps {
    fn default() -> Self {
        Self {
            name: "Ferris the 🦀".to_string(),
        }
    }
}
