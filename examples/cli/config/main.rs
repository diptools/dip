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

#[config_plugin]
#[derive(Debug, Deserialize)]
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
