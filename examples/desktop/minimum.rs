use dip::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Minimum Example".to_string(),
            ..Default::default()
        })
        .add_plugin(DesktopPlugin::<NoUiState, NoUiAction, NoAsyncAction>::new(
            Root,
        ))
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello, World !" }
    })
}
