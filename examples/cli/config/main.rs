use dip::prelude::*;
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};

fn main() {
    let config = SimpleConfig::new();

    App::new()
        // Provide config struct via type parameter
        .insert_resource(config.build::<MyConfig>(
            // Path to a user defined config file
            &PathBuf::from_str("examples/cli/config/config/development").unwrap(),
            // Default file to be included in binary
            include_str!("config/default.toml"),
        ))
        .add_system(log_config)
        .run();
}

fn log_config(config: Res<MyConfig>) {
    println!("{:#?}", *config);
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MyConfig {
    base_url: String,
    backend: Backend,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Backend {
    api_key: String,
    api_secret: String,
}
