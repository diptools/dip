<div align="center">
    <h1>bevy_dioxus</h1>
    <p>
        <a href="https://github.com/DioxusLabs/dioxus/" target="_blank"> Dioxus </a> Plugin for <a href="https://github.com/bevyengine/bevy" target="_blank">Bevy</a>
    </p>
    <p>Write Cross-platform application with React-like decralative UI framework<br/>and scalable ECS architecture all in Rust.</p>
</div>

<br/>

```rust
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Dioxus Plugin Example".to_string(),
            ..Default::default()
        })
        .add_plugin(DioxusPlugin::<(), ()>::new(Root))
        .run();
}

fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello, World !" }
    })
}
```

### Try examples

```sh
gh repo clone JunichiSugiura/bevy_dioxus

cargo run --example counter
```

More examples can be found in [examples/](https://github.com/JunichiSugiura/bevy_dioxus/tree/main/examples) directory.
