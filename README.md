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
    <p>Write cross-platform application with React-like declarative UI framework<br/>and scalable ECS architecture all in Rust.</p>
    <p align="center">
     <a href="https://docs.rs/bevy_dioxus/latest/bevy_dioxus/" alt="API Refenrence">
        <img src="https://img.shields.io/badge/API Reference-000?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" />
     </a>
     <!-- Link to Guide -->
    </p>
</div>

<br/>


> WARNING: `bevy_dioxus` is still in the very early stages of development.

```rust, no_run
use bevy::prelude::*;
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Dioxus Plugin Example".to_string(),
            ..Default::default()
        })
        .add_plugin(DioxusPlugin::<EmptyGlobalState, (), ()>::new(Root))
        .run();
}

fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello, World !" }
    })
}
```

## About Dioxus and Bevy
### Dioxus
Dioxus is a cross-platform declarative UI framework. It provides familiar features that React developer expects such as component, state, props, hooks, global state, and router. If you familiar with any modern state driven UI framework, you should be able to read or write Dioxus components without knowing Rust. 

### Bevy
Bevy is a cutting-edge game engine in Rust based on Entity Component System(ECS) design pattern. Think of it as a global state management tool like Redux but much more performant because all systems will run concurrently as much as possible. Thanks to its plugin system, there's already a handlfull of third-party Bevy plugins out there. Imagine implemnenting core logic as `CorePlugin` seperated from UI layer. You may start with `bevy_dioxus` to build desektop application. Then let's say you want to release a metaverse edition at some point in the future, it's as simple as swapping UI plugin to Bevy's 3d rendering plugin while still using the same CorePlugin.

## Try examples

Make sure to install all prerequisites for Tauri.
[Prerequisites](https://tauri.studio/v1/guides/getting-started/prerequisites)

```sh
gh repo clone JunichiSugiura/bevy_dioxus
cd bevy_dioxus

cargo run --example counter
```

More examples can be found in [examples/](https://github.com/JunichiSugiura/bevy_dioxus/tree/main/examples) directory.

## Development

### Prerequisites
#### General
- [Tauri prerequisites](https://tauri.studio/v1/guides/getting-started/prerequisites)
- [convco](https://github.com/convco/convco#installation): Conventional commits, changelog, versioning, validation
  ```sh
  cargo install convco
  # or
  brew install convco/formulae/convco
  ```
- [cargo-workspaces](https://github.com/pksunkara/cargo-workspaces): A tool for managing cargo workspaces and their crates, inspired by lerna
  ```sh
  cargo install cargo-workspaces
  ```

#### Website
- [Zola](https://github.com/getzola/zola): A fast static site generator in a single binary with everything built-in.
  ```sh
  brew install zola
  ```
- [Node.js](https://nodejs.org/en/download/): To install Tailwind CSS

#### API Reference
- [cargo-watch](https://github.com/watchexec/cargo-watch): Watches over your Cargo project's source.
  ```sh
  cargo install cargo-watch
  ```

### Run
#### Examples
```sh
cargo run --example counter
```

#### Website
```sh
# Install dependencies
npm i

# Serve locally
zola -r packages/website serve

# Watch Tailwind CSS
npm run watch

# or build
npm run build
```

#### API Reference
```sh
# Serve doc locally
cargo doc --open --no-deps

# Watch file changes and serve doc locally
cargo watch -s 'cargo doc && http target/doc'
```

### Conventions

#### Branch name example
```sh
git checkout -b docs/#20-guide-website
```

#### Conventional Commits
Make sure to use `convco commit` instead of `git commit` when it should be noted in changelog. [git-cliff](https://github.com/orhun/git-cliff) will automatically generates changelog automatically based on conventional-commit message that convco produces.
```sh
convco commit
```
