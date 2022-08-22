+++
title = "Overview"
weight = 0
+++

## Prerequisites
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

## Run
```sh
# Build
cargo build --examples

# or Run
cargo run --example counter
```
