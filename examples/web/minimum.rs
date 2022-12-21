use dip::prelude::*;

fn main() {
    App::new().add_plugin(WebPlugin::new(Root)).run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello, WASM !" }
    })
}
