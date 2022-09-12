<div align="center">
    <h1>dip</h1>
    <p align="center">
        <a href="https://github.com/JunichiSugiura/dip/actions/workflows/rust.yml" alt="Github Actions">
            <img src="https://img.shields.io/github/workflow/status/JunichiSugiura/dip/Rust?style=for-the-badge&logo=github" />
        </>
        <a href="https://docs.rs/dip/latest/dip/" alt="API Docs">
            <img src="https://img.shields.io/docsrs/dip?style=for-the-badge" />
        </a>
        <a href="https://crates.io/crates/dip" alt="Crates.io Page">
            <img src="https://img.shields.io/crates/v/dip?style=for-the-badge" />
        </a>
        <img src="https://img.shields.io/crates/d/dip?style=for-the-badge" />
        <img src="https://img.shields.io/crates/l/dip?style=for-the-badge" />
    </p>
    <p>Previous called bevy_dioxus.</p>
    <p>Write cross-platform application with React-like declarative UI<br/>and scalable ECS architecture all in Rust.</p>
    <p align="center">
        <a href="https://www.dip.tools" alt="Website">
            <img src="https://img.shields.io/badge/Website-000?style=for-the-badge" />
        </a>
        <a href="https://www.dip.tools/docs/getting-started/overview/" alt="Documentation">
            <img src="https://img.shields.io/badge/Documentation-000?style=for-the-badge" />
        </a>
        <a href="https://docs.rs/dip/latest/dip/" alt="API Refenrence">
            <img src="https://img.shields.io/badge/API Reference-000?style=for-the-badge&logo=docsdotrs" />
        </a>
    </p>
</div>

<br/>


> WARNING: `dip` is still in the very early stages of development.

> `main` branch is currently preparing for v0.2 release.

```rust, no_run
use dip::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "dip Plugin Example".to_string(),
            ..Default::default()
        })
        .add_plugin(DesktopPlugin::<NoUiState, NoUiAction>::new(Root))
        .run();
}

fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello, World !" }
    })
}
```

## Features

#### Desktop App

<details>
<summary>Code example</summary>

```toml
# Cargo.toml

[dependencies]
dip = { version = "0.2", features = ["desktop"] }
```

```rust, no_run
use dip::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Desktop App".to_string(),
            ..Default::default()
        })
        .add_plugin(DesktopPlugin::<NoUiState, NoUiAction>::new(Root))
        .run();
}

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
```

</details>

#### CLI App

<details>
<summary>Code example</summary>

```toml
# Cargo.toml

[dependencies]
dip = { version = "0.2", features = ["cli"] }
clap = { version = "3.2", features = ["derive"] }
```

```rust, no_run
use dip::{bevy::log::LogPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugin(CliPlugin)
        .add_plugin(LogPlugin)
        .add_system(log_root_arg)
        .add_system(log_path_flag)
        .add_system(handle_hello)
        .add_system(handle_task)
        .add_system(handle_ping)
        .run();
}

#[derive(CliPlugin, clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    root_arg: Option<String>,

    #[clap(short, long)]
    path: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Subcommand, Clone)]
enum Commands {
    Hello { name: Option<String> },
    Task(TaskArgs),
    Ping,
}

#[derive(clap::Args, Debug, Clone)]
struct TaskArgs {
  name: String
}

fn log_root_arg(cli: Res<Cli>) {
    if let Some(arg) = &cli.root_arg {
        info!("root arg: {:?}", arg);
    }
}

fn log_path_flag(cli: Res<Cli>) {
    if let Some(path) = &cli.path {
        info!("path flag: {:?}", path);
    }
}

fn handle_hello(mut events: EventReader<Hello>) {
    for e in events.iter() {
        info!("Hello, {}!", e.name.clone().unwrap_or("world".to_string()));
    }
}

fn handle_task(mut events: EventReader<Task>) {
    for e in events.iter() {
        info!("{e:?}");
    }
}

fn handle_ping(mut events: EventReader<Ping>) {
    for _ in events.iter() {
        info!("Pong !");
    }
}
```

```sh
cargo run -- --help

dip-cli-example 0.1.0
Junichi Sugiura
Example binary project to showcase CliPlugin usage.

USAGE:
    cli [OPTIONS] [ROOT_ARG] <SUBCOMMAND>

ARGS:
    <ROOT_ARG>

OPTIONS:
    -h, --help           Print help information
    -p, --path <PATH>
    -V, --version        Print version information

SUBCOMMANDS:
    hello
    help     Print this message or the help of the given subcommand(s)
    ping
    task

```
</details>

#### State management

<details>
<summary>Code example</summary>

```toml
# Cargo.toml

[dependencies]
dip = { version = "0.2", features = ["desktop"] }

# Removing this crate throws error.
# This is because some derive macros generates code using sub crate name instead of root
# (e.x. bevy_ecs::Component vs bevy::ecs::Compoent)
bevy_ecs = "0.8"
```

```rust, no_run
use dip::prelude::*;

fn main() {
    App::new()
        // Step 7. Put it all together
        .add_plugin(DesktopPlugin::<UiState, UiAction>::new(Root))
        .add_plugin(UiStatePlugin) // generated by #[ui_state]
        .add_plugin(UiActionPlugin) // generated by #[ui_action]
        .add_system(update_name)
        .run();
}

// Step 1: Define UiState
#[ui_state]
struct UiState {
    name: Name,
}

#[derive(Clone, Debug)]
pub struct Name {
    value: String,
}

impl Default for Name {
    fn default() -> Self {
        Self {
            value: "world".to_string(),
        }
    }
}

// Step 2. Define actions
#[derive(Clone, Debug)]
pub struct UpdateName {
    value: String,
}

// Step 3. Implement action creators
#[ui_action]
impl ActionCreator {
    fn update_name(value: String) -> UpdateName {
        UpdateName { value }
    }
}

// Step 4. Implement systems to handle each action defined in step 2
fn update_name(mut events: EventReader<UpdateName>, mut name: ResMut<Name>) {
    for action in events.iter() {
        name.value = action.value.clone();
    }
}

fn Root(cx: Scope) -> Element {
    // Step 5. Select state
    let name = use_read(&cx, NAME);

    let window = use_window::<UiAction>(&cx);

    cx.render(rsx! {
        h1 { "Hello, {name.value} !" }

        input {
            value: "{name.value}",
            oninput: |e| {
                // Step 6. Dispatch the action !
                window.send(UiAction::update_name(e.value.to_string()));
            },
        }
    })
}
```

</details>

#### Keyboard handling
- [Keyboard event](https://github.com/diptools/dip/blob/main/examples/keyboard/keyboard_event.rs)
- [Key bindings](https://github.com/diptools/dip/blob/main/examples/keyboard/bindings.rs)

## About Bevy and Dioxus
### Bevy
[https://github.com/bevyengine/bevy](https://github.com/bevyengine/bevy)
- Data-driven game engine based on Entity Component System(ECS) design pattern
- Flexible Plugin design
- Plugin ecosystem

Bevy is a cutting-edge game engine in Rust based on Entity Component System(ECS) design pattern. Think of it as a global state management tool like Redux but much more performant because all systems will run concurrently as much as possible. Thanks to its plugin system, there's already a handlfull of third-party Bevy plugins out there. Imagine implemnenting core logic as `CorePlugin` seperated from UI layer. You may start with `dip::desktop` to build desektop application. Then let's say you want to release a metaverse edition at some point in the future, it's as simple as swapping UI plugin to Bevy's 3d rendering plugin while still using the same CorePlugin.

### Dioxus
[https://github.com/DioxusLabs/dioxus](https://github.com/DioxusLabs/dioxus)
- Cross-platform (macOS, Linux, Windows, TUI, etc.)
- React-like declarative UI library
- Virtual dom is 3x faster than React
- Minimum bundle size is around 20x lighter than Electron (8 MB vs 160MB)

Dioxus is a cross-platform declarative UI library. It provides familiar features that React developer expects such as component, state, props, hooks, global state, and router. If you familiar with any modern state driven UI framework, you should be able to read or write Dioxus components without knowing Rust. 
## Examples
Make sure to install all prerequisites for Tauri.
[Prerequisites](https://tauri.studio/v1/guides/getting-started/prerequisites)

Find more in [examples/](https://github.com/diptools/dip/tree/main/examples) directory.

```sh
gh repo clone diptools/dip
cd dip

# Counter example
cargo run --example counter --features desktop

# TodoMVC example
npm install # requires npm for styling

# this script compiles Tailwind CSS and starts Rust example
cargo run --example todomvc --features desktop
```

## Milestone
[📌 dip - Project board](https://github.com/orgs/diptools/projects/1)
