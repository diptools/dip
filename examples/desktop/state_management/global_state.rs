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

static NAME: Atom<String> = |_| "world".to_string();

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let name = use_read(&cx, NAME);
    let set_name = use_set(&cx, NAME);

    cx.render(rsx! {
        h1 { "Hello, {name} !" }

        input {
            value: "{name}",
            oninput: |e| {
                set_name(e.value.to_string());
            },
        }
    })
}
