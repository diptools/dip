use dip::prelude::*;
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugin(ConfigPlugin)
        .add_system(log_config)
        .run();
}

fn log_config(config: Res<Config>) {
    println!("{:#?}", *config);
}

#[derive(Debug, Deserialize)]
#[config_plugin(
    env_prefix = "APP", // BACKEND__API_KEY -> APP__BACKEND__API_KEY
    env_separator = "___", // BACKEND___API_KEY -> BACKEND___API_KEY
    // defualt look up paths for user_config are
    // ./{CARGO_PKG_NAME}
    // $HOME/.{CARGO_PKG_NAME}
    // $HOME/.config/{CARGO_PKG_NAME}/{CARGO_PKG_NAME}
    // $HOME/.config/{CARGO_PKG_NAME}
    override_user_config_path = "examples/cli/config/config/development",
    override_user_config_path = "path/to/user/config/file2"
)]
#[allow(dead_code)]
struct Config {
    base_url: String,
    backend: Backend,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Backend {
    api_key: String,
    api_secret: String,
}
