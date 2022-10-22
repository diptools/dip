## What is Bevy?

- [GitHub](https://github.com/bevyengine/bevy)
- Data-driven game engine based on Entity Component System(ECS) design pattern
- Flexible Plugin design
- Plugin ecosystem

Bevy is a cutting-edge game engine in Rust based on Entity Component System(ECS) design pattern. Think of it as a global state management tool like Redux but much more performant because all systems will run as parallel as possible. Thanks to its plugin system, there's already a handful of third-party Bevy plugins out there. Imagine implementing core logic as `CorePlugin` separated from UI layer. You may start with `dip::desktop` to build desktop application. Then let's say you want to release a metaverse edition at some point in the future, it's as simple as swapping UI plugin to Bevy's 3d rendering plugin while still using the same CorePlugin.

## What is Dioxus?

- [GitHub](https://github.com/DioxusLabs/dioxus)
- Cross-platform (macOS, Linux, Windows, TUI, etc.)
- React-like declarative UI library
- Virtual dom is 3x faster than React
- Minimum bundle size is around 20x lighter than Electron (8 MB vs 160MB)

Dioxus is a cross-platform declarative UI library. It provides familiar features that React developer expects such as component, state, props, hooks, global state, and router. If you familiar with any modern state driven UI framework, you should be able to read or write Dioxus components without knowing Rust. 

## Is this library intended to be used with Bevy games?
- [Original Github Discussion](https://github.com/diptools/dip/discussions/51]

>So the short answer is no unfortunately. You cannot overlay UI within the same window that renders game graphics. This is because dip depends on a different window manager called [TAO](https://github.com/tauri-apps/tao). Which is actually the hard fork of [winit](https://github.com/rust-windowing/winit) but totally different rendering logic. 
> 
>At least for now, this project is aiming for DOM based app developers that needs extra power and flexibility in their apps comparing to existing solution like Electron + React stack.
