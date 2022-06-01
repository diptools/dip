<div align="center">
    <h1>bevy_dioxus</h1>
    <p align="center">
        <a href="https://github.com/JunichiSugiura/bevy_dioxus/actions/workflows/rust.yml" alt="Github Actions">
            <img src="https://img.shields.io/github/workflow/status/JunichiSugiura/bevy_dioxus/Rust?style=for-the-badge&logo=github" />
        </>
        <a href="https://docs.rs/bevy_dioxus/latest/bevy_dioxus/" alt="API Docs">
            <img src="https://img.shields.io/docsrs/bevy_dioxus?style=for-the-badge" />
        </a>
        <a href="https://crates.io/crates/bevy_dioxus" alt="Crates.io Page">
            <img src="https://img.shields.io/crates/v/bevy_dioxus?style=for-the-badge" />
        </a>
        <img src="https://img.shields.io/crates/d/bevy_dioxus?style=for-the-badge" />
        <img src="https://img.shields.io/crates/l/bevy_dioxus?style=for-the-badge" />
    </p>
    <p>
        <a href="https://github.com/DioxusLabs/dioxus/" target="_blank"> Dioxus </a> Plugin for <a href="https://github.com/bevyengine/bevy" target="_blank">Bevy</a>
    </p>
    <p>Write Cross-platform application with React-like decralative UI framework<br/>and scalable ECS architecture all in Rust.</p>
</div>

<br/>


> WARNING: `bevy_dioxus` is still in the very early stages of development.

```rust
use bevy::prelude::*;
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

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

## Try examples

```sh
gh repo clone JunichiSugiura/bevy_dioxus
cd bevy_dioxus

cargo run --example counter
```

More examples can be found in [examples/](https://github.com/JunichiSugiura/bevy_dioxus/tree/main/examples) directory.


## Development

```sh
# serve doc locally
cargo doc --open --no-deps

# watch file changes
cargo watch -s 'cargo doc && http target/doc'
```

