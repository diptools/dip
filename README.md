<div align="center">
    <h1>bevy_dioxus</h1>
    <p>Dioxus Plugin for Bevy</p>
    <p>Write Cross-platform application with React like decralative UI framework and scalable ECS architecture all in Rust.</p>
</div>

<br/>

```rust
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Dioxus Plugin Example".to_string(),
            ..Default::default()
        })
        .add_plugin(DioxusPlugin::<CoreCommand, UiCommand>::new(app, ()));
        .run();
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello, World !" }
    })
}
```
