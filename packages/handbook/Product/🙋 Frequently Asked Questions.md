### 🙋 Frequently Asked Questions

- [[#Is this library intended to be used with Bevy games?]]
- [[#What is Bevy?]]
- [[#What is Dioxus?]]
- [[#What is Obsidian?]]

## Is this library intended to be used with Bevy games?

- [Original Github Discussion](https://github.com/diptools/dip/discussions/51)

>So the short answer is no unfortunately. You cannot overlay UI within the same window that renders game graphics. This is because dip depends on a different window manager called [TAO](https://github.com/tauri-apps/tao). Which is actually the hard fork of [winit](https://github.com/rust-windowing/winit) but totally different rendering logic. 
> 
>At least for now, this project is aiming for DOM based app developers that needs extra power and flexibility in their apps comparing to existing solution like Electron + React stack.

## What is Bevy?

![[🛒 Glossary#Bevy]]

## What is Dioxus?

![[🛒 Glossary#Dioxus]]

## What is Obsidian?

![[🛒 Glossary#Obsidian]]
