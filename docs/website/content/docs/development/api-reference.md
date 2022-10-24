+++
title = "API Reference"
weight = 1
+++

## Prerequisites
- [cargo-watch](https://github.com/watchexec/cargo-watch): Watches over your Cargo project's source.
  ```sh
  cargo install cargo-watch
  ```

## Run
```sh
# Serve doc locally
cargo doc --open --no-deps

# Watch file changes and serve doc locally
cargo watch -s 'cargo doc && http target/doc'
```
