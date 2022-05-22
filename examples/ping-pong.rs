use bevy::{log::LogPlugin, prelude::*};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Ping-Pong Example".to_string(),
            ..Default::default()
        })
        .add_plugin(DioxusPlugin::<CoreCommand, UiCommand>::new(Root))
        .add_plugin(LogPlugin)
        .add_system(pong)
        .run();
}

#[derive(Debug, Clone)]
enum CoreCommand {
    Ping,
}

#[derive(Debug, Clone)]
enum UiCommand {
    Pong,
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let window = use_window::<CoreCommand, UiCommand>(&cx);

    use_future(&cx, (), |_| {
        let rx = window.receiver();

        async move {
            while let Some(cmd) = rx.receive().await {
                match cmd {
                    UiCommand::Pong => {
                        info!("🎨 Pong");
                    }
                }
            }
        }
    });

    cx.render(rsx! {
        h1 { "Ping Pong Example" }
        p { "💡 Press \"Ping\" and see console." }
        button {
            onclick: move |_| window.send(CoreCommand::Ping),
            "Ping"
        }
    })
}

fn pong(mut events: EventReader<CoreCommand>, mut ui: EventWriter<UiCommand>) {
    for cmd in events.iter() {
        info!("🧠 {:?}", cmd);
        ui.send(UiCommand::Pong);
    }
}
