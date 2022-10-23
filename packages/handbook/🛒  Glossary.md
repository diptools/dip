### Bevy

- [GitHub](https://github.com/bevyengine/bevy)
- Data-driven game engine based on Entity Component System(ECS) design pattern
- Flexible Plugin design
- Plugin ecosystem

Bevy is a cutting-edge game engine in Rust based on Entity Component System(ECS) design pattern. Think of it as a global state management tool like Redux but much more performant because all systems will run as parallel as possible. Thanks to its plugin system, there's already a handful of third-party Bevy plugins out there. Imagine implementing core logic as `CorePlugin` separated from UI layer. You may start with `dip::desktop` to build desktop application. Then let's say you want to release a metaverse edition at some point in the future, it's as simple as swapping UI plugin to Bevy's 3d rendering plugin while still using the same CorePlugin.

### Dioxus

- [GitHub](https://github.com/DioxusLabs/dioxus)
- Cross-platform (macOS, Linux, Windows, TUI, etc.)
- React-like declarative UI library
- Virtual dom is 3x faster than React
- Minimum bundle size is around 20x lighter than Electron (8 MB vs 160MB)

Dioxus is a cross-platform declarative UI library. It provides familiar features that React developer expects such as component, state, props, hooks, global state, and router. If you familiar with any modern state driven UI framework, you should be able to read or write Dioxus components without knowing Rust. 

### Obsidian

[What is Obsidian](https://help.obsidian.md/Obsidian/Obsidian#What+is+Obsidian)

> Obsidian is a both a Markdown editor and a knowledge base app.
> 
> Used in the most basic way, you can edit and preview Markdown files. But its true power lies in managing densely networked knowledge base.
> 
> How do we start creating a network, you ask? Let's first start making some