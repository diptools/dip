use dip::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Local State".to_string(),
            ..Default::default()
        })
        .add_plugin(DesktopPlugin::<NoUiState, NoUiAction>::new(Root))
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let name = use_state(&cx, || "world".to_string());

    cx.render(rsx! {
        h1 { "Hello, {name} !" }

        input {
            value: "{name}",
            oninput: |e| {
                name.set(e.value.to_string());
            },
        }
    })
}
